use rustic_rover3::common::{Logger, ProcessConnector};
use rustic_rover3::common::type_define::{LogType, Packet, ProcessInfo, Robot};

use std::net::UdpSocket;

fn main()
{
    let mut recver = ProcessConnector::new(64208);
    let logger = Logger::new();
    let mut info = ProcessInfo::new("UdpTransporter", LogType::Info, "Start UdpTransporter");

    let external_socket = UdpSocket::bind("192.168.11.55:64201").unwrap();
    let _ = external_socket.set_read_timeout(Some(std::time::Duration::from_millis(1000)));

    logger.log(info.clone());

    let mut dest_ip = String::new();
    let mut check = false;
    while !check
    {
        let mut buf = [0_u8; 256];

        match external_socket.recv_from(&mut buf) {
            Ok((size, addr))=>{
                info.status = LogType::Info;

                let read_data = &buf[..size];
                let read_str = String::from_utf8_lossy(read_data).to_string();

                info.message = format!("Get Connection from {}:{}", addr.ip().to_string(), addr.port());
                logger.log(info.clone());
                info.message = format!("Message -> {}", read_str);   
                logger.log(info.clone());  
                
                check = true;           
                dest_ip = format!("{}:{}", addr.ip().to_string(), addr.port());
            }
            Err(_e)=>{
                info.message = format!("Seatch RusticRover-Client...");
                info.status = LogType::Warn;
                logger.log(info.clone());
            }
        }
    }

    loop {
        let recv = recver.recv::<Robot>();

        match recv {
            Some(send_data)=>{
                match external_socket.send_to(send_data.serialization().as_bytes(), dest_ip.as_str()) {
                    Ok(_size)=>{
                        info.status = LogType::Info;
                        info.message = format!("Transport -> {},{},{},{},{},{}", send_data.wheel1,send_data.wheel2,send_data.wheel3, send_data.frontback,send_data.updown,send_data.hand);
                        logger.log(info.clone());
                    }
                    Err(_e)=>{
                        info.status = LogType::Error;
                        info.message = format!("Failed to transport data.");
                    }
                }
            }
            None=>{

            }
        }

        // std::thread::sleep(std::time::Duration::from_millis(50));
    }
}