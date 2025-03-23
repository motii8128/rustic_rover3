use rustic_rover3::common::Logger;
use rustic_rover3::common::type_define::{DoubleController, GameController, LogType, Packet, ProcessInfo};
use gamepads::{Gamepads, Button};
use std::net::UdpSocket;

fn main()
{
    let sock = UdpSocket::bind("192.168.11.50:64201").unwrap();
    let mut gamepads = Gamepads::new();

    let logger = Logger::new();
    let mut info = ProcessInfo::new("GamePadDriver", LogType::Info, "Start GamePadDriver");

    let mut controller_num = 0;

    logger.log(info.clone());

    loop {
        gamepads.poll();
        let mut controller_data = DoubleController::new();
        let mut controllers = Vec::<GameController>::new();

        for gamepad in gamepads.all()
        {
            let mut controller = GameController::new();

            controller.left_stick.x = gamepad.left_stick_x();
            controller.left_stick.y = gamepad.left_stick_y();
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

        match sock.send_to(controller_data.gc1.serialization().as_bytes(), "192.168.11.2:64205")
        {
            Ok(_size)=>{
                info.message = "Send Message".to_string();
                logger.log(info.clone());
            }
            Err(_e)=>{

            }
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