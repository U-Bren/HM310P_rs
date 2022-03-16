use tokio_modbus::client::{Context, Reader};
use crate::sku::Feature;
use tokio_modbus::prelude::Writer;
use futures::executor::block_on;

/*
pub struct PSU {
    pub context: Context,
    #[deprecated]
    pub features: Vec<Feature>, 
    pub hackish_features: Vec<Feature>,
}
*/

pub struct Command {
    pub feature: Feature,
    pub parameters: u16 //TODO: Support Vec<u16>
}


pub trait ControllablePSU {
    fn new(context: Context) -> Self;
    fn read(&mut self, command: &Command) -> Result<Vec<u16>, std::io::Error>;
    fn write(&mut self, command: &Command) -> std::result::Result<(), std::io::Error>;
    fn features(&self) -> &Vec<Feature>;
    fn get_feature(&self, feature_name: &str) -> Option<Feature> {
        self.features().clone().into_iter()
            .filter(|x| x.name == feature_name)
            .next()
    }
}


pub fn read(context: &mut  Context, command: &Command) -> Result<Vec<u16>, std::io::Error> {
    block_on(
        context.read_input_registers(command.feature.address+1, command.feature.quantity)
    ) //+1 because of protocol.
}

pub fn write(context: &mut  Context, command: &Command) -> std::result::Result<(), std::io::Error> {
    //TODO: Support Vec<u16>
    block_on(
        context.write_single_register(
                command.feature.address+1, // +1 because of protocol
                command.parameters)
    ) }

/*
impl ControllablePSU for PSU {
    fn send(&mut self, command: &Command) -> std::result::Result<(), std::io::Error> {
        use tokio_modbus::prelude::Writer;
        use futures::executor::block_on;
        
        block_on(self.context.write_single_register(command.feature.address+1,command.parameters.u16)) //+1 because of protocol.
    }

}
*/