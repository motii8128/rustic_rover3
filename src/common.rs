pub mod type_define;


use std::net::UdpSocket;
use std::time::Instant;
use colored::Colorize;

use type_define::{LogType, Packet, ProcessInfo};
pub struct ProcessConnector
{
    sock : UdpSocket,
}

impl ProcessConnector {
    pub fn new(port : u16)->Self
    {
        let addr = format!("127.0.0.1:{}", port);

        let sock_ = UdpSocket::bind(addr).unwrap();

        let _ = sock_.set_read_timeout(Some(std::time::Duration::from_millis(1000)));
        Self { sock: sock_}
    }

    pub fn send<T : Packet>(&mut self, dest_port : u16, value : T)->bool
    {
        let addr = format!("127.0.0.1:{}", dest_port);
        match self.sock.send_to(value.serialization().as_bytes(), addr) {
            Ok(_size)=>{
                true
            }
            Err(_e)=>{
                false
            }
        }
    }

    pub fn recv<T:Packet>(&mut self)->Option<T>
    {
        let mut buf = [0_u8; 512];
        match self.sock.recv(&mut buf) {
            Ok(size)=>{
                let get_data = &buf[..size];
                let string_data = String::from_utf8_lossy(get_data).to_string();

                Some(T::deserialization(string_data))
            }
            Err(_e)=>{
                None
            }
        }
    }
}

pub struct Logger
{
    timer : Instant
}
impl Logger {
    pub fn new()->Self
    {
        Self { timer: Instant::now() }
    }

    pub fn log(&self, info : ProcessInfo)
    {
        let content = format!("[{}][{}]:{}", info.name, self.timer.elapsed().as_secs().to_string(), info.message);

        match info.status {
            LogType::Info=>{
                println!("{}", content.green())
            }
            LogType::Warn=>{
                println!("{}", content.yellow())
            }
            LogType::Error=>{
                println!("{}", content.red())
            }
        }
    }
}