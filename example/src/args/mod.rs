use crate::ParsingErr::PortErr;
use clap::crate_version;
use clap::Parser;
use psu::psu::Command;
use psu::psu::PSU;
use psu::sku::RWCapabilities;
use psu::sku::hm310p::HM310P;
use psu::sku::Feature;
use tracing::error;
use tracing::trace;
use std::fmt;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tokio_modbus::client;
use tokio_modbus::prelude::*;
use tokio_modbus::client::Context;
use tokio_serial::{DataBits, ErrorKind, SerialPortBuilder, StopBits};
use snafu::prelude::*;
use crate::RawOpts;

#[derive(Debug)]
pub struct Opts {
    pub psu: Box<PSU>,
    pub command: Command,
}

#[derive(Debug, Snafu)]
pub enum ParsingErr {
    #[snafu(display("Cannot find such a SKU."))]
    NoSuchSKU,
    #[snafu(display("Cannot use port: {}", source.description))]
    PortErr { source: tokio_serial::Error },
    #[snafu(display("Cannot resolve command"))]
    NoSuchCommand,
    #[snafu(display("Connection to port failed: {}", source))]
    PortConnectionFailed { source: std::io::Error },
}

/// This should only be called once.
pub async fn get_opts(rawOpts: &RawOpts) -> Result<Opts, ParsingErr> {
    trace!("{:#?}", rawOpts);
    use ParsingErr::*;

    let port = match get_serial_port(rawOpts.port.as_str()) {
        Ok(serial) => serial,

        Err(e) => {
            return Err(PortErr{source: e});
        }
    };

    let context: tokio_modbus::client::Context = match rtu::connect_slave(port, Slave(rawOpts.slave_id)).await {
        Err(e) => return Err(PortConnectionFailed{source: e}),
        Ok(x) => x
    };

    let psu = match get_psu(&rawOpts.sku, context) {
        None => return Err(NoSuchSKU),
        Some(x) => x,
    };

    let command = match parse_command(&psu, &rawOpts.command, RWCapabilities::from_str(&rawOpts.mode).unwrap()) {
        None => return Err(NoSuchCommand),
        Some(x) => x,
    };

    Ok(Opts { psu, command })
}


use tokio_serial::SerialStream;
pub fn get_serial_port(tty_path: &str) -> Result<SerialStream, tokio_serial::Error> {
    let builder = tokio_serial::new(tty_path, 9600);
    SerialStream::open(&builder)
}

use std::io::Error;
pub async fn build_context(port: SerialStream, slave_id: u8) -> Result<Context, Error>{
    rtu::connect_slave(port, Slave(slave_id)).await
}

fn get_psu(sku: &str, context: Context) -> Option<Box<PSU>> {
    match sku.to_lowercase().as_str() {
        "autodetect" => todo!("Autodetection of sku is not implemented yet."),
        "hm310p" => Some(Box::new(
            HM310P::new(context).psu
        )),
        _ => {
            error!("Unrecognized SKU: {}", sku);
            None
        }
    }
}

fn parse_command(psu: &PSU, command: &str, mode: RWCapabilities) -> Option<Command> {
    //raw_feature: String = command.split

    let feature = match psu
        .features
        .iter()
        .filter(|x| x.name.eq_ignore_ascii_case(command))
        .next()
    {
        None => return None,
        Some(x) => *x,
    };

    let parameters: u16 = 0x00000;

    Some(Command {
        mode,
        feature,
        parameters,
    })
}


/*impl fmt::Display for ParsingErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortErr(e) => {
                write!(f, "A error with the port has occured.");
                use ErrorKind::*;
                match e {
                    NoDevice => write!(f, "Error: Can't find port."),
                    InvalidInput => write!(f, "Error: Invalid input."),
                    Unknown => write!(f, "Error: Unknown error for port."),
                    Io(io) => match io {
                        std::io::ErrorKind::PermissionDenied => write!(f, "Error: Permission denied for port."),
                        _ => write!(f, "Error: IO error.")
                    },
                    //std::io::ErrorKind::ResourceBusy => println!("Error: Resource \"{}\" Busy.", opts.port),
                    _ => write!(f, "Error: Can't use open port."),
                }
            }
            NoSuchSKU => write!(f, "."),
            NoSuchCommand => write!(f, ),
        }
    }
}*/