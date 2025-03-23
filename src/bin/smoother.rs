use rustic_rover3::common::{Logger, ProcessConnector};
use rustic_rover3::common::type_define::{LogType, ProcessInfo, Cmd};

fn main()
{
    let mut recver = ProcessConnector::new(64204);
    let mut sender = ProcessConnector::new(64205);
    let logger = Logger::new();
    let mut info = ProcessInfo::new("Smoother", LogType::Info, "Start Smoother");
    logger.log(info.clone());

    let wheel_gain = 0.1;
    let machine_gain = 0.5;

    let mut prev = Cmd::new();

    loop {
        let recv = recver.recv::<Cmd>();

        match recv {
            Some(target)=>{
                let mut send = Cmd::new();

                send.x = smooth(target.x, prev.x, wheel_gain);
                send.y = smooth(target.y, prev.y, wheel_gain);
                send.rotation = smooth(target.rotation, prev.rotation, wheel_gain);
                send.updown = smooth(target.updown, prev.updown, machine_gain);
                send.frontback = smooth(target.frontback, prev.frontback, machine_gain);
                send.hand = smooth(target.hand, prev.hand, machine_gain);

                sender.send(64206, send.clone());

                // info.message = format!("{},{},{}", send.x, send.y, send.rotation);
                // info.status = LogType::Info;
                // logger.log(info.clone());

                prev = send;
                
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            None=>{
                info.message = format!("Failed to receive controller data.");
                info.status = LogType::Error;
                logger.log(info.clone());
            }
        }
    }
}

fn smooth(target : f32, prev :f32, gain :f32)->f32
{
    let vec = target - prev;
    if vec > 0.0
    {
        if vec > gain
        {
            prev + gain
        }
        else
        {
            target
        }
    }
    else if vec < 0.0
    {
        if vec.abs() > gain
        {
            prev - gain
        }
        else {
            target
        }
    }
    else {
        target
    }
}