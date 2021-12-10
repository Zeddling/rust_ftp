use std::env;

pub enum Instance {
    Client,
    Server,
}

/**
 * Stores cmd arguments
 */
pub struct Cmd {
    /**
     * Type of instance. Can be server or client
     */
    instance_type: Instance,

    port: String,
}

impl Cmd {
    pub fn create(args: Vec<String>) -> Cmd {
        let i_type = &args[1];
        
        match i_type.to_ascii_lowercase().as_str() {
            "server" => Cmd {
                instance_type: Instance::Server,
                port: String::new(),
            },
            "client" => {
                let port = args[2].to_string();
                Cmd {
                    instance_type: Instance::Client,
                    port: port,
                }
            }
            _ => panic!("Unknown command")
        }

    }

    pub fn instance_type(&self) -> &Instance {
        &self.instance_type
    }

    pub fn port(&self) -> &String {
        &self.port
    }

}

/**
 * Gets cmd arguments, validates them and returns a string
 */
pub fn get_cmd_arguments() -> Cmd {
    let args: Vec<String> = env::args().collect();

    Cmd::create(args)
}
