use anyhow::bail;
use serialport::{SerialPortInfo, SerialPortType};

pub fn find_port() -> anyhow::Result<SerialPortInfo> {
    for port in serialport::available_ports()? {
        if let SerialPortType::UsbPort(info) = &port.port_type {
            if info.vid == 0x1fc9 && info.pid == 0x0132 {
                // Found the serial port provided by the debugger on the
                // LPC845-BRK board.
                return Ok(port);
            }
        }
    }

    bail!("Serial port of LP845-BRK not found")
}
