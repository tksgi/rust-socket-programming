use mio::tcp::{TcpListener, TcpStream};
use mio::{Event, Events, Poll, PollOpt, Ready, Token};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::{env, process, str};
#[macro_use]
extern crate log;


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
            next_connection_id: 1,
            })
    }

}


fn main() {
    println!("Hello, world!");
}
