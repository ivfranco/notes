mod echo;
mod prethread;

// use echo::echo_server;
use prethread::echo_server;

fn main() {
    if let Err(e) = echo_server() {
        eprintln!("{}", e);
    }
}
