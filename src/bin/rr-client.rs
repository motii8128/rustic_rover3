use std::net::UdpSocket;
use rustic_rover3::common::type_define::{LogType, Packet, ProcessInfo, Robot};
use rustic_rover3::common::Logger;
use serialport;

fn main()
{
    let logger = Logger::new();

    let external_socket = UdpSocket::bind("192.168.11.62:64201").unwrap();

    // let mut serial = serialport::new("/dev/ttyACM0", 230400).open().unwrap();

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

                let mut buf = [0_u8; 7];
                buf[0] = ((cmd.wheel1 as f32 * 1.27) + 127.0) as u8;
                buf[1] = ((cmd.wheel2 as f32 * 1.27) + 127.0) as u8;
                buf[2] = ((cmd.wheel3 as f32 * 1.27) + 127.0) as u8;
                buf[3] = ((cmd.frontback as f32 * 1.27) + 127.0) as u8;
                buf[4] = ((cmd.updown as f32 * 1.27) + 127.0) as u8;
                buf[5] = ((cmd.hand as f32 * 1.27) + 127.0) as u8;
                buf[6] = b'\n';

                info.message = format!("Data -> {},{},{},{},{},{}", buf[0],buf[1],buf[2],buf[3],buf[4],buf[5]);
                logger.log(info.clone());

                // match serial.write(&buf) {
                //     Ok(_size)=>{
                //         info.message = format!("Data -> {},{},{},{},{},{}", buf[0],buf[1],buf[2],buf[3],buf[4],buf[5]);
                //         logger.log(info.clone());
                //     }
                //     Err(_e)=>{

                //     }
                // }

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(_e)=>{

            }
        }
    }
}