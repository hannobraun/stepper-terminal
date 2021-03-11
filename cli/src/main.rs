mod args;
mod serial;

use clap::Clap as _;

use self::args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let port = serial::find_port()?;
    println!("LPC845-BRK port found: {}", port.port_name);

    Ok(())
}
