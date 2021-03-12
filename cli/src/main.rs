mod args;
mod serial;

use clap::Clap as _;

use self::args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let port = serial::find_port()?;
    let mut port = serialport::new(port.port_name, 115_200).open()?;
    writeln!(port, "Hello, world!")?;

    Ok(())
}
