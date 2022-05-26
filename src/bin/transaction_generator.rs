use clap::Parser;
use rand::*;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long)]
    record_count: u32,
}
fn main() {
    let args = Arguments::parse();
    let nubmer_generator = rand::RandomNumberGenerator();
    for record in 0..args.record_count {
        let generated_transaction = generate_transaction(number_generator);

        writeln!()
    }
}
