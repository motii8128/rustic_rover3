use std::io;
use termion::input::TermRead;

pub struct Launcher
{
    nodes : Vec<String>,
    process : Vec<std::process::Child>
}
impl Launcher {
    pub fn new()->Self
    {
        Launcher { nodes: Vec::new() , process : Vec::new()}
    }

    pub fn add(&mut self, node : &str)
    {
        self.nodes.push(node.to_string());
    }

    pub fn launch(&mut self)
    {
        for node in self.nodes.clone()
        {
            let _process = std::process::Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg(node)
                .spawn().unwrap();
            
            self.process.push(_process);
        }

        let stdin = io::stdin();

        for c in stdin.keys()
        {
            match c {
                Ok(termion::event::Key::Ctrl('c'))=>{
                    let pro_nm = self.process.len();

                    for i in 0..pro_nm
                    {
                        let _ = self.process[i].kill();
                    }
                }
                Err(_)=>{

                }
                _=>{}
            }
        }
    }
}