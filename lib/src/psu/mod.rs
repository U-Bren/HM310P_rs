use crate::sku::{Feature, RWCapabilities};
use futures::executor::block_on;
use tokio_modbus::client::{Context, Reader};
use tokio_modbus::prelude::Writer;
use tracing::error;

//FIXME: Validate mode against feature's is_rw
#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub mode: RWCapabilities,
    pub feature: Feature,
    pub parameters: u16, //TODO: Support Vec<u16>
}

#[derive(Debug)]
pub struct PSU {
    pub context: Context,
    pub features: Vec<Feature>,
}

impl PSU {
    pub async fn read(&mut self, command: &Command) -> Result<Vec<u16>, std::io::Error> {
        error!("Attempting a read with command {:#?}.", command);
            self.context.read_holding_registers(command.feature.address + 1, command.feature.quantity).await //+1 because of protocol
    }

    pub async fn write(&mut self, command: &Command) -> std::result::Result<(), std::io::Error> {
        error!("Attempting a write with command {:#?}.", command);
        //TODO: Support Vec<u16>
        self.context.write_single_register(
            command.feature.address + 1, // +1 because of protocol
            command.parameters,
        ).await
    }

    pub fn get_feature(&self, feature_name: &str) -> Option<Feature> {
        self.features
            .clone()
            .into_iter()
            .find(|x| x.name == feature_name)
    }
/*
    /***
     * High level warpper for #write & #read
     */
    //FIXME: Find a way to return useful result
    pub async fn send_command(&mut self, command: &Command) -> Result<Result<Vec<u16>, std::io::Error>, ()> {
        match command.mode {
            RWCapabilities::Read => self.read(command).await,
            RWCapabilities::Write => self.write(command).await,
        }
    }*/
}
