use serde::{Serialize, Deserialize};
use serde_json;

pub trait Packet
where 
    Self: Serialize + for<'de> Deserialize<'de> + Sized + Clone,
{
    fn serialization(&self)->String
    {
        serde_json::to_string(self).unwrap()
    }

    fn deserialization(str : String)->Self
    {
        serde_json::from_str(&str).unwrap()
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Axis
{
    pub x: f32,
    pub y: f32,
}
impl Packet for Axis{}
impl Axis {
    pub fn new()->Self
    {
        Axis { x: 0.0, y: 0.0 }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Button
{
    pub circle : i8,
    pub triangle : i8,
    pub cube : i8,
    pub cross : i8,
    pub r1 : i8,
    pub r2 : i8,
    pub l1 : i8,
    pub l2 : i8
}
impl Packet for Button {}
impl Button {
    pub fn new()->Self
    {
        Button { circle: 0, triangle: 0, cube: 0, cross: 0, r1: 0, r2: 0, l1: 0, l2: 0 }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameController
{
    pub left_stick : Axis,
    pub right_stick : Axis,
    pub dpad : Axis,
    pub btns : Button
}
impl Packet for GameController {}
impl GameController {
    pub fn new()->Self
    {
        GameController { left_stick: Axis::new(), right_stick: Axis::new(), dpad: Axis::new(), btns: Button::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DoubleController
{
    pub num : u8,
    pub gc1 : GameController,
    pub gc2 : GameController
}
impl Packet for DoubleController {}
impl DoubleController {
    pub fn new()->Self
    {
        Self { num: 0, gc1: GameController::new(), gc2: GameController::new() }
    }
}

#[derive(Serialize, Deserialize,Clone)]
pub enum LogType
{
    Info = 1,
    Warn = 2,
    Error = 3
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessInfo
{
    pub name : String,
    pub status : LogType,
    pub message : String,
}
impl Packet for ProcessInfo {}

impl ProcessInfo {
    pub fn new(name : &str, status : LogType, message : &str)->Self
    {
        Self { name: name.to_string(), status : status, message: message.to_string() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Robot
{
    pub wheel1:i32,
    pub wheel2:i32,
    pub wheel3:i32,
    pub frontback:i32,
    pub updown:i32,
    pub hand:i32
}
impl Packet for Robot {}
impl Robot {
    pub fn new()->Self
    {
        Self { wheel1: 0, wheel2: 0, wheel3: 0, frontback: 0, updown: 0, hand: 0 }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cmd
{
    pub x : f32,
    pub y : f32,
    pub rotation : f32,
    pub frontback : f32,
    pub updown : f32,
    pub hand : f32
}
impl Packet for Cmd {}
impl Cmd {
    pub fn new()->Self
    {
        Self { x: 0.0, y: 0.0, rotation: 0.0, frontback: 0.0, updown: 0.0, hand: 0.0 }
    }
}