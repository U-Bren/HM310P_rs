use std::thread::sleep;
use std::time::Duration;
use tokio_serial::{DataBits, StopBits};
use core::array;

#[derive(Debug)]
struct Address {
    address: u16,
    name: &'static str,
    is_rw: bool,
    quantity: u16,
}

const ADDRESS_TABLE: [Address;14] = [
    Address {
        address: 0x0000,
        name: "ON/OFF",
        is_rw: true,
        quantity: 0,
    },
    Address {
        address: 0x0001,
        name: "OP.S",
        is_rw: false,
        quantity: 0,
    },
    Address {
        address: 0x0002,
        name: "Specification and type",
        is_rw: false,
        quantity: 0,
    },
    Address {
        address: 0x0003,
        name: "Tail classification",
        is_rw: false,
        quantity: 0,
    },
    Address {
        address: 0x0004,
        name: "Decimal point digit capacity",
        is_rw: false,
        quantity: 0,

    },
    Address {
        address: 0x0005,
        name: "U",
        is_rw: false,
        quantity: 2,

    },
    Address {
        address: 0x0006,
        name: "I",
        is_rw: false,
        quantity: 2,

    },
    Address {
        address: 0x0007,
        name: "P",
        is_rw: false,
        quantity: 3,

    },
    Address {
        address: 0x0008,
        name: "SetU",
        is_rw: true,
        quantity: 2,

    },
    Address {
        address: 0x0009,
        name: "SetI",
        is_rw: true,
        quantity: 2,

    },
    Address {
        address: 0x000A,
        name: "OVP",
        is_rw: true,
        quantity: 2,

    },
    Address {
        address: 0x000B,
        name: "OCP",
        is_rw: true,
        quantity: 3,

    },
    Address {
        address: 0x000C,
        name: "OPP",
        is_rw: true,
        quantity: 2,

    },
    Address {
        address: 0x000D,
        name: "RS-Adder",
        is_rw: true,
        quantity: 0,
    }
];

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_serial::SerialStream;

    use tokio_modbus::prelude::*;

    let tty_path = "COM9";
    let slave = Slave(0x01);

    let builder = tokio_serial::new(tty_path, 9600)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);

    let port = SerialStream::open(&builder).unwrap();

    let mut ctx = rtu::connect_slave(port, slave).await?;
    println!("Reading a sensor value");
    let mut is_powered: bool = false;
    let mut address = 0x0000;

    println!("{:?}", ADDRESS_TABLE.iter().filter(|x| x.name == "ON/OFF").next().unwrap());
    println!("{:?}", ADDRESS_TABLE.iter().filter(|x| x.name == "U").next().unwrap());
    println!("{:?}", ADDRESS_TABLE.iter().filter(|x| x.name == "I").next().unwrap());
    println!("{:?}", ADDRESS_TABLE.iter().filter(|x| x.name == "SetU").next().unwrap());

    /*loop {
        let rsp = ctx.read_holding_registers(address, 2).await?;
        println!("{:?}={:?}", address, rsp);
        if address == 0xFFFF {
            sleep(Duration::from_secs(2));
        }
        address += 1;
    }*/
    Ok(())
}
