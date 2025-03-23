use rustic_rover3::common::{Logger, ProcessConnector};
use rustic_rover3::common::type_define::{DoubleController, LogType, ProcessInfo, Cmd};

fn main()
{
    let mut gc_recver = ProcessConnector::new(64202);
    let mut cmd_sender = ProcessConnector::new(64203);
    let logger = Logger::new();

    let mut core_info = ProcessInfo::new("CommandCreator", LogType::Info, "Start CommandCreator.");
    logger.log(core_info.clone());

    loop {
        let recv = gc_recver.recv::<DoubleController>();
        match recv {
            Some(con)=>{
                core_info.status = LogType::Info;
                let mut cmd = Cmd::new();
                cmd.x = fix(con.gc1.left_stick.x);

                cmd.y = fix(con.gc1.left_stick.y);

                cmd.rotation = fix(con.gc1.right_stick.x);

                if con.num == 1
                {
                    cmd.updown = con.gc1.dpad.y;
                    cmd.frontback = con.gc1.dpad.x;
                    cmd.hand = con.gc1.btns.r1 as f32 - con.gc1.btns.l1 as f32;
                }
                else if con.num == 2
                {
                    cmd.updown = con.gc2.dpad.y;
                    cmd.frontback = con.gc2.dpad.x;
                    cmd.hand = con.gc2.btns.r1 as f32 - con.gc2.btns.l1 as f32;
                }
                
                cmd_sender.send(64204, cmd.clone());

                // core_info.message = format!("{},{},{}", cmd.x, cmd.y, cmd.rotation);
                // logger.log(core_info.clone());

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            None=>{
                core_info.message = format!("Failed to receive controller data.");
                core_info.status = LogType::Error;
                logger.log(core_info.clone());
            }
        }
    }
}

fn fix(value : f32)->f32
{
    if value.abs() < 0.05
    {
        0.0
    }
    else if value.abs() > 0.99
    {
        if value > 0.0
        {
            1.0
        }
        else {
            -1.0
        }
    }
    else {
        value
    }
}