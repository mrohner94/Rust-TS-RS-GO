use clap::Parser;
use Rust_TS_RS_GO:: {opts::Opts, config::Config};
use anyhow::Result;
fn main() -> Result<()> {
    // let opts = Rust_TS_RS_GO::opts::Opts::parse();
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(())
}