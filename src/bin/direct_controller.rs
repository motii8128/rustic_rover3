use rustic_rover3::common::Logger;
use rustic_rover3::common::type_define::{DoubleController, GameController, LogType, ProcessInfo};
use gamepads::{Gamepads, Button};
use serialport;

fn main()
{
    let mut gamepads = Gamepads::new();

    let mut serial = serialport::new("/dev/ttyACM0", 115200).open().unwrap();


    let logger = Logger::new();
    let mut info = ProcessInfo::new("GamePadDriver", LogType::Info, "Start DirectDriver");

    let mut controller_num = 0;

    logger.log(info.clone());

    let mut last = std::time::Instant::now();

    let mut pos = 0.0_f64;

    loop {
        gamepads.poll();
        let mut controller_data = DoubleController::new();
        let mut controllers = Vec::<GameController>::new();

        for gamepad in gamepads.all()
        {
            let mut controller = GameController::new();

            controller.left_stick.x = gamepad.left_stick_x();
            controller.left_stick.y = gamepad.left_stick_y();
            if controller.left_stick.y.abs() < 0.05
            {
                controller.left_stick.y = 0.0;
            }
            controller.right_stick.x = gamepad.right_stick_x();
            controller.right_stick.y = gamepad.right_stick_y();

            controller.dpad.x = bool_to_f32(gamepad.is_currently_pressed(Button::DPadRight)) - bool_to_f32(gamepad.is_currently_pressed(Button::DPadLeft));
            controller.dpad.y = bool_to_f32(gamepad.is_currently_pressed(Button::DPadUp)) - bool_to_f32(gamepad.is_currently_pressed(Button::DPadDown));

            controller.btns.circle = bool_to_i8(gamepad.is_currently_pressed(Button::ActionRight));
            controller.btns.triangle = bool_to_i8(gamepad.is_currently_pressed(Button::ActionUp));
            controller.btns.cube = bool_to_i8(gamepad.is_currently_pressed(Button::ActionLeft));
            controller.btns.cross = bool_to_i8(gamepad.is_currently_pressed(Button::ActionDown));

            controller.btns.r1 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontRightUpper));
            controller.btns.r2 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontRightLower));
            controller.btns.l1 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontLeftUpper));
            controller.btns.l2 = bool_to_i8(gamepad.is_currently_pressed(Button::FrontLeftLower));

            

            controller_data.num += 1;
            controllers.push(controller);            
        }

        match controllers.get(0) {
            Some(con)=>{
                controller_data.gc1 = con.clone();
            }
            None=>{

            }
        }

        match controllers.get(1) {
            Some(con)=>{
                controller_data.gc2 = con.clone()
            }
            None=>{

            }
        }

        if controller_num != controller_data.num
        {
            controller_num = controller_data.num;
            if controller_num == 1
            {
                info.message = format!("1 controller is connected.");
                info.status = LogType::Warn;
                logger.log(info.clone());
                info.message = format!("Single Entry Mode");
                logger.log(info.clone());
            }
            else if controller_num == 2
            {
                info.message = format!("2 controller is connected.");
                info.status = LogType::Warn;
                logger.log(info.clone());
                info.message = format!("Double Entry Mode");
                logger.log(info.clone());
            }
            else if controller_num == 0
            {
                info.message = format!("controller is not connected.");
                info.status = LogType::Error;
                logger.log(info.clone());
                info.message = format!("Stop Robot Control");
                logger.log(info.clone());
            }
            
        }

        pos += (controller_data.gc1.btns.triangle - controller_data.gc1.btns.cross)  as f64 * 0.000001 as f64;
        if pos > 1.0
        {
            pos = 1.0
        }
        else if pos < 0.0
        {
            pos = 0.0
        }

        let mut buf = [0_u8; 7];
        buf[0] = ((controller_data.gc1.left_stick.y * 127.0) as i16 + 127) as u8;
        buf[1] = ((controller_data.gc1.dpad.y * 127.0) as i16 + 127) as u8;
        buf[2] = (((controller_data.gc1.btns.l1 - controller_data.gc1.btns.l2) as f32 * 127.0) as i16 + 127) as u8;
        buf[3] = ((pos * 127.0) as i16 + 127) as u8;
        buf[4] = ((controller_data.gc1.right_stick.y as f32 * 127.0) as i16 + 127) as u8;
        buf[5] = (((controller_data.gc1.btns.r1 - controller_data.gc1.btns.r2)as f32 * 127.0) as i16 + 127) as u8;
        buf[6] = b'\n';


        if last.elapsed() >= std::time::Duration::from_millis(30)
        {
            match serial.write(&buf)
            {
                Ok(_size)=>{
                    info.message = format!("Position : {:.5}", pos);
                    logger.log(info.clone());
                }
                Err(_e)=>{

                }
            }

            last = std::time::Instant::now();
        }
    }
}

fn bool_to_f32(b : bool)->f32
{
    if b
    {
        1.0
    }
    else 
    {
        0.0
    }
}

fn bool_to_i8(b: bool)->i8
{
    if b
    {
        1
    }
    else {
        0
    }
}