//  Import server and client implementations
mod client;
mod server;
mod utils;

fn main() {
    println!("Starting application...");

    let cmd = utils::get_cmd_arguments();

    match cmd.instance_type() {
        utils::Instance::Server => {
            let s = server::new();
            s.run();
        },
        utils::Instance::Client => {
            client::run(format!("127.0.0.1:{}", cmd.port()))
        }
    };

}
