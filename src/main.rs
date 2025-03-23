use rustic_rover3::common::Logger;
use rustic_rover3::common::type_define::{ProcessInfo, LogType};
use rustic_rover3::launcher::Launcher;

fn main() {
    let logger = Logger::new();
    let mut info = ProcessInfo::new("RusticRover3", LogType::Info, "Initalize Launcher");

    let mut launcher = Launcher::new();
    logger.log(info.clone());

    launcher.add("game_controller");
    info.message = format!("Set GameControllerNode");
    logger.log(info.clone());

    launcher.add("command_creator");
    info.message = format!("Set CommandCreatorNode");
    logger.log(info.clone());

    launcher.add("smoother");
    info.message = format!("Set SmootherNode");
    logger.log(info.clone());

    launcher.add("vel_calculator");
    info.message = format!("Set VelocityCalculatorNode");
    logger.log(info.clone());

    launcher.add("udp_transporter");
    info.message = format!("Set UdpTransporter");
    logger.log(info.clone());

    launcher.launch();
}
