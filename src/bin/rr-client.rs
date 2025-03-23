use std::net::UdpSocket;
use rustic_rover3::common::type_define::{LogType, Packet, ProcessInfo, Robot};
use rustic_rover3::common::Logger;
use serialport;

fn main()
{
    let logger = Logger::new();

    let external_socket = UdpSocket::bind("192.168.11.62:64201").unwrap();

    // let serial = serialport::new("/dev/ttyACM0", 230400).open().unwrap();

    let mut info = ProcessInfo::new("rr-client", LogType::Info, "Start RusticRover-Client");
    logger.log(info.clone());

    loop {
        let send_msg = format!("rr-client in Orin Nano");
        match external_socket.send_to(send_msg.as_bytes(), "192.168.11.55:64201") {
            Ok(_size)=>{
                
            }
            Err(_e)=>{

            }
        }

        let mut buf = [0_u8;256];
        match external_socket.recv(&mut buf) {
            Ok(size)=>{
                info.message = format!("Receive Message");
                logger.log(info.clone());

                let read_data = &buf[..size];
                let read_str = String::from_utf8_lossy(read_data).to_string();

                let cmd = Robot::deserialization(read_str);

                info.message = format!("Data -> {},{},{},{},{},{}", cmd.wheel1,cmd.wheel2,cmd.wheel3,cmd.frontback,cmd.updown,cmd.hand);
                logger.log(info.clone());
            }
            Err(_e)=>{

            }
        }
    } 
}