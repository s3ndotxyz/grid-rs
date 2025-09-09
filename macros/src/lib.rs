use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    if sig.asyncness.is_some() {
        return quote!(compile_error!("async functions are not supported")).into();
    }

    let orig_ident = sig.ident.clone();
    let new_ident = Ident::new(&format!("__{orig_ident}"), sig.ident.span());
    sig.ident = new_ident.clone();

    let main_impl = quote_spanned! {sig.output.span()=>
        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn call() -> i32 {
            let result = std::panic::catch_unwind(|| {
                let input_data = {
                    let ptr = grid_rs::Input::read_all();
                    if ptr.is_null() {
                        return Err("Failed to read input".to_string());
                    }
                    unsafe { grid_rs::region::Region::consume(ptr) }
                };

                #new_ident(&input_data)
            });

            match result {
                Ok(Ok(data)) => grid_rs::Output::write_all(&data) as i32,
                Ok(Err(e)) => {
                    let error = serde_json::json!({ "error": e });
                    grid_rs::Output::write_all(&serde_json::to_vec(&error).unwrap()) as i32
                },
                Err(e) => {
                    let error = serde_json::json!({
                        "error": format!("Runtime panic: {:?}", e)
                    });
                    grid_rs::Output::write_all(&serde_json::to_vec(&error).unwrap()) as i32
                }
            }
        }

        fn main() {
            // Native main that does nothing but satisfy the compiler
            let input = std::env::args().nth(1).unwrap_or_default();
            let result = #new_ident(input.as_bytes());
            match result {
                Ok(data) => println!("{}", String::from_utf8_lossy(&data)),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    };

    quote! {
        #(#attrs)*
        #vis #sig #block

        #main_impl
    }
    .into()
}
