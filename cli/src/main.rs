mod args;
mod serial;

use anyhow::anyhow;
use clap::Clap as _;

use self::args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let port = serial::find_port()?;
    let mut port = serialport::new(port.port_name, 115_200).open()?;

    let command: protocol::Command = args.command.into();

    let mut buf = [0; 1024];
    let command = postcard::to_slice_cobs(&command, &mut buf)
        .map_err(|err| anyhow!("Error encoding with Postcard: {:?}", err))?;

    port.write_all(command)?;

    Ok(())
}
