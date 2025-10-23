//! Real-time WebSocket.
//!
//! This module provides WebSocket functionality for both client and server connections.
//! It currently supports TCP with plans to support TLS connections (including support for RA certs), with automatic resource cleanup.
//!
//! # Examples
//!
//! ## Server
//! ```rust
//! use grid_rs::ws::WebSocketServer;
//!
//! let ws = WebSocketServer::create("127.0.0.1:9002")?;
//! if let Some(conn) = ws.accept()? {
//!     if let Some(msg) = conn.receive()? {
//!         conn.send(conn.id, &msg)?; // Echo back
//!     }
//! }
//! ```
//!
//! ## Client
//! ```rust
//! use grid_rs::ws::WebSocketClient;
//!
//! let ws = WebSocket::connect("ws://localhost:9002")?;
//! ws.send(b"Hello WebSocket!")?;
//! if let Some(response) = ws.receive()? {
//!     return Ok(response.to_vec());
//! }
//! ```
//! See chat room [example](https://github.com/s3ndotxyz/grid-rs/tree/main/examples/websocket) for
//! more complex usage.
//!

use crate::region::Region;
use std::io::Result;

unsafe extern "C" {
    fn ws_server_create() -> usize;
    fn ws_server_accept(server_id: usize) -> usize;
    fn ws_server_send(server_id: usize, conn_id: usize, data_ptr: usize);
    fn ws_server_receive(server_id: usize, conn_id: usize) -> usize;
    fn ws_server_close(server_id: usize);

    fn ws_client_connect(url_ptr: usize) -> usize;
    fn ws_client_send(client_id: usize, data_ptr: usize);
    fn ws_client_receive(client_id: usize) -> usize;
    fn ws_client_close(client_id: usize);
}

#[derive(Debug)]
pub struct WebSocketServer {
    pub id: u32,
}

#[derive(Debug)]
pub struct WebSocketClient {
    pub id: u32,
}

#[derive(Debug)]
pub struct WebSocketConnection {
    pub id: u32,
    pub server_id: u32,
}

impl WebSocketServer {
    pub fn create() -> Result<Self> {
        let id = unsafe { ws_server_create() } as u32;
        Ok(Self { id })
    }

    pub fn accept(&self) -> Result<Option<WebSocketConnection>> {
        let conn_id = unsafe { ws_server_accept(self.id as usize) } as u32;
        if conn_id == 0 {
            return Ok(None);
        }
        Ok(Some(WebSocketConnection { id: conn_id, server_id: self.id }))
    }

    pub fn close(&self) -> Result<()> {
        unsafe { ws_server_close(self.id as usize) }
        Ok(())
    }
}

impl WebSocketClient {
    pub fn connect(url: &str) -> Result<Self> {
        let url = Region::build(url.as_bytes());
        let url_ptr = &*url as *const Region;
        let id = unsafe { ws_client_connect(url_ptr as usize) } as u32;
        Ok(Self { id })
    }

    pub fn send(&self, data: &[u8]) -> Result<()> {
        let data = Region::build(data);
        let data_ptr = &*data as *const Region;

        unsafe { ws_client_send(self.id as usize, data_ptr as usize) }
        Ok(())
    }

    pub fn receive(&self) -> Result<Option<Vec<u8>>> {
        let data_ptr = unsafe { ws_client_receive(self.id as usize) };
        if data_ptr == 0 {
            return Ok(None);
        }
        unsafe { Ok(Some(Region::consume(data_ptr as *mut Region))) }
    }

    pub fn close(&self) -> Result<()> {
        unsafe { ws_client_close(self.id as usize) }
        Ok(())
    }
}

impl WebSocketConnection {
    pub fn send(&self, client_id: u32, data: &[u8]) -> Result<()> {
        let data = Region::build(data);
        let data_ptr = &*data as *const Region;
        unsafe { ws_server_send(self.server_id as usize, client_id as usize, data_ptr as usize) }
        Ok(())
    }

    pub fn receive(&self) -> Result<Option<Vec<u8>>> {
        let data_ptr = unsafe { ws_server_receive(self.server_id as usize, self.id as usize) };
        if data_ptr == 0 {
            return Ok(None);
        } else if data_ptr == 1 {
            return Err(std::io::Error::other("Connection closed"));
        }
        unsafe { Ok(Some(Region::consume(data_ptr as *mut Region))) }
    }
}
