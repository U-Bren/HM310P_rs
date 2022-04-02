mod args;
use crate::ParsingErr::PortErr;
use clap::crate_version;
use clap::Parser;
use psu::psu::Command;
use psu::psu::PSU;
use psu::sku::hm310p::HM310P;
use psu::sku::Feature;
use tracing::Level;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use tokio_modbus::prelude::*;
use tokio_serial::{DataBits, ErrorKind, SerialStream, StopBits};
use snafu::prelude::*;
use args::{get_opts, ParsingErr, Opts};

#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = "U_Bren <ruben@caramail.com>")]
pub struct RawOpts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    port: String,
    #[clap(short='i', long, default_value="1")]
    slave_id: u8,
    #[clap(short, long, default_value="autodetect")]
    sku: String,
    #[clap(short, long)]
    command: String,
    #[clap(short, long, default_value="read")]
    mode: String,
    #[clap(short='V',long)]
    value: u16,
    #[clap(short, long, parse(from_occurrences))]
    verbose: u32,
}



#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let raw_opts: RawOpts = RawOpts::parse();
    let mut opts = parse_opts(&raw_opts).await;

    debug!("Available features: N/A because printing a vec is a nightmare");

    let command = Command {
        mode: opts.command.mode,
        feature: opts.psu.get_feature("ON/OFF").unwrap(),
        parameters: raw_opts.value,
    };

    info!("Attempting to send command...");
    let vec = opts.psu.write(&command).await;
    info!("Result: todo!");
    
    Ok(())
}

fn init_logging() {
    // Install color backtrace as a panic handler
    color_backtrace::install();

    // Send subscribed information to stdout
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

/// Returns parsed Opts
/// 
/// # Arguments
/// * `rawOpts` - a RawOpts instance usually parsed using Clap from CLI args.
/// 
/// # Exits
/// With error code ``1`` when creating a Opts instance from passed ``rawOpts`` fails.
async fn parse_opts(rawOpts: &RawOpts) -> Opts {
    let opts = match get_opts(&rawOpts).await {
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        },
        Ok(x) => x
    };

    debug!(
        "Parsed opts: {:#?}",    /*, command {}"*/
        &opts, /*opts.command*/
    );

    opts
}