use clap::Parser;

fn main() {
    let opts = Rust_TS_RS_GO::opts::Opts::parse();
    println!("{:?}", opts)
}