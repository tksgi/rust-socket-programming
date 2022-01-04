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
    fn new(addr: &str) -> Result<Self, failure::Error> {
        let address = addr.parse()?;
        let listening_socket = TcpListener::bind(&address)?;
        Ok(WebServer {
            listening_socket,
            connections: HashMap::new(),
            next_connection_id: 1,
        })
    }

    /**
     * サーバーを起動
     */
    fn run(&mut self) -> Result<(), failure::Error> {
        let poll = Poll::new()?;
        // サーバーソケットの状態を監視対象に登録
        poll.register(
            &self.listening_socket,
            SERVER,
            Ready::readable(),
            PollOpt::level(),
        )?;

        // イベントキュー
        let mut events = Events::with_capacity(1024);
        // HTTPのレスポンス用バッファ
        let mut response = Vec::new();

        loop {
            // 現在のスレッドをブロックしてイベントを待つ
            match poll.poll(&mut events, None) {
                Ok(_) => {}
                Err(e) => {
                    error!("{ }", e);
                    continue;
                }
            }
            for event in &events {
                match event.token() {
                    SERVER => {
                        // リスニングソケットの読み込み準備完了イベントが発生
                        let (stream, remote) = match self.listening_socket.accept() {
                            Ok(t) => t,
                            Err(e) => {
                                error!("{ }", e);
                                continue;
                            }
                        };
                        debug!("Connection from { }", &remote);
                        // 接続済みソケットを監視対象に登録
                        self.register_connection(&poll, stream)
                            .unwrap_or_else(|e| error!("{ }", e));
                    }

                    Token(conn_id) => {
                        // 接続済みソケットでイベント発生
                        self.http_handler(conn_id, event, &poll, &mut response)
                            .unwrap_or_else(|e| error!("{  }", e));
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
