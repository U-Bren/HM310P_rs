use std::thread::sleep;
use std::time::Duration;
use clap::{Parser,crate_version};
use psu::sku::hm310p::HM310P;
use tokio_serial::{DataBits, ErrorKind, StopBits};
use core::{array, panic};
use psu::sku::{self, Feature, Command};
use psu::psu::{PSU, Controllable};
use tokio_serial::SerialStream;
use tokio_modbus::prelude::*;

#[derive(Parser)]
#[clap(version = crate_version!(), author = "U_Bren <ruben@caramail.com>")]
struct RawOpts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    port: String,
    #[clap(short, long)]
    sku: String,
    #[clap(short, long)]
    hex: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: u32,
}

struct Opts {
    port: SerialStream,
    sku: SKU,
    command: Command 
}


#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let rawOpts: RawOpts = RawOpts::parse();
    println!("Port: {}, command {}", rawOpts.port, rawOpts.hex);

}

fn parse_args(rawOpts: RawOpts) -> Opts {
    let port: Option<SerialStream> = match get_serial_port(rawOpts.port.as_str()){
        Ok(serial) => Some(serial),

        Err(e) => {
            use ErrorKind::*;
            match e.kind {
                NoDevice => println!("Error: Can't find port \"{}\".", rawOpts.port),
                InvalidInput => println!("Error: Invalid input \"{}\".", rawOpts.port),
                Unknown => println!("Error: Unknown error for port {}.", rawOpts.port),
                Io(io) => match io {
                    std::io::ErrorKind::PermissionDenied => println!("Error: Permission denied for port \"{}\".", rawOpts.port),
                    //std::io::ErrorKind::ResourceBusy => println!("Error: Resource \"{}\" Busy.", opts.port),
                    _ => println!("Error: Can't use open port \"{}\".", rawOpts.port),
                },
            }
            None
        }
    };

    let sku: SKU = parse_sku(rawOpts.sku);

    let command: Command = parse_command(rawOpts.hex);

    Opts{
        command,
        sku,
        port,
    }
}

fn parse_sku(sku: String) -> Controllable {
    match sku {
        "hm310p" => HM310P
        _ => panic!("Can't find PSU's SKU")
    }
}

async fn yolo() -> Result<(), Box<dyn std::error::Error>> {
    let port = get_serial_port("COM9")?;
    let slave = Slave(0x01);

    let context = rtu::connect_slave(port, slave).await?;
    let mut sku = HM310P::new(context);
    let features: Vec<Feature> = sku.psu.features.clone();

    
    let on_off = (features.into_iter().filter(|x| x.name == "ON/OFF").next()).unwrap();

    let mut flip = false;
    loop {
        println!("{:?}", flip);
        sku.psu.send(&on_off, u16::from(flip));
        println!("sent");
        flip = !flip;
        sleep(Duration::from_secs(1));
    }
}

fn get_serial_port(tty_path: &str) -> tokio_serial::Result<SerialStream> {

    let builder = tokio_serial::new(tty_path, 9600)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);

    let serial = SerialStream::open(&builder)?;
    Ok(serial)
}