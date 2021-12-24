use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

/*
* 指定のソケットアドレスで接続を待ち受ける
*/
pub fn serve(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;
        // スレッドを立ち上げて接続に対処
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error)); // {:?} は配列を展開して表示
        });
    }
}

/*
* クライアントからの入力待ち受け、受信したらオウム返し
*/
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024]; // 符号なし8-bit整数型;
    loop {
        let nbytes = stream.read(&mut buffer)?;

        // stream.readはConnectionが切断されると0を返す
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
