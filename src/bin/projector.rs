use clap::Parser;
use rust_ts_rs_go:: {opts::Opts, config::Config};
use anyhow::Result;
fn main() -> Result<()> {
    // let opts = rust_ts_rs_go::opts::Opts::parse();
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(())
}