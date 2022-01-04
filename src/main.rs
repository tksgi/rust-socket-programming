extern crate mio;
extern crate regex;

use std::net::SocketAddr;
use std::{ env, str, fs };
use std::io::{ Error, Read, BufReader, Write };
use std::collections::HashMap;

use mio::*;
use mio::tcp::{ TcpListener, TcpStream };
use regex::Regex;


const SERVER: Token = Token(0);
const WEBROOT: &str = "/webroot";


struct WebServer {
    listening_socket: TcpListener,
    connections: HashMap<usize, TcpStream>, // サーバーに接続されているクライアントをハッシュで管理
    next_connection_id: usize,
}

impl WebServer {
    /**
    * サーバーの初期化
    */
    fn new(addr: &str) -> Result<Self, failure:Error> {
        let address = addr.parse()?;
        let listening_socket = TcpListener::bind(&address)?;
        Ok(WebServer {
            listening_socket,
            connections: HashMap::new(),
            next_connection_id,
            })
    }

}


fn main() {
    println!("Hello, world!");
}
