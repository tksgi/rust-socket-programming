use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::{self, MutableTcpPacket, TcpFlags};
use pnet::transport::{
    self, TransportChannelType, TransportProtocol, TransportReceiver, TransportSender,
};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::{env, fs, process, thread};
#[macro_use]
extern crate log;

const TCP_SIZE: usize = 20;

fn main() {
    println!("Hello, world!");
}
