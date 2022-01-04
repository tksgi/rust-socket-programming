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


fn main() {
    println!("Hello, world!");
}
