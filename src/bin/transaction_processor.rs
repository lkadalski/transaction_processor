use std::env;
use transaction_processor::handler;
fn main() {
    let input = env::args().nth(1);
    if let Err(err) = handler::input::handle(input) {
        eprintln!("Error occured during processing: {err}");
        std::process::exit(1)
    }
}
