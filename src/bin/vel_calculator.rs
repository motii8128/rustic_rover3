use rustic_rover3::common::{Logger, ProcessConnector};
use rustic_rover3::common::type_define::{LogType, ProcessInfo, Robot, Cmd};

fn main()
{
    let mut gc_recver = ProcessConnector::new(64206);
    let mut cmd_sender = ProcessConnector::new(64207);
    let logger = Logger::new();

    let mut info = ProcessInfo::new("VelocityCalculator", LogType::Info, "Start VelocityCalculator.");
    logger.log(info.clone());

    let wheel_rad1 = 150.0_f32.to_radians();
    let wheel_rad2 = 30_f32.to_radians();
    let wheel_rad3 = 270.0_f32.to_radians();
    let radius = 1.0;

    loop {
        let recv = gc_recver.recv::<Cmd>();
        match recv {
            Some(con)=>{
                info.status = LogType::Info;
                let mut cmd = Robot::new();
                let wheel1_rate = -1.0*wheel_rad1.sin()*con.x + wheel_rad1.cos()*con.y + radius * con.rotation;
                cmd.wheel1 = (100.0 * wheel1_rate) as i32;

                let wheel2_rate = -1.0*wheel_rad2.sin()*con.x + wheel_rad2.cos()*con.y + radius * con.rotation;
                cmd.wheel2 = (100.0 * wheel2_rate) as i32;

                let wheel3_rate = -1.0*wheel_rad3.sin()*con.x + wheel_rad3.cos()*con.y + radius * con.rotation;
                cmd.wheel3 = (100.0 * wheel3_rate) as i32;

                cmd.frontback = con.frontback as i32 * 100;
                cmd.updown = con.updown as i32 * 100;
                cmd.hand = con.hand as i32 * 100;

                if cmd.wheel1.abs() < 7
                {
                    cmd.wheel1 = 0;
                }
                if cmd.wheel2.abs() < 7
                {
                    cmd.wheel2 = 0;
                }
                
                cmd_sender.send(64208, cmd.clone());
            }
            None=>{
                info.message = format!("Failed to receive controller data.");
                info.status = LogType::Error;
                logger.log(info.clone());
            }
        }

        // std::thread::sleep(std::time::Duration::from_millis(50));
    }
}