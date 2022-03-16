use crate::{sku::Feature, psu::{ControllablePSU, self, Command}};
use tokio_modbus::client::Context;

pub struct HM310P {
    context: Context,
    features: Vec<Feature>
}

impl ControllablePSU for HM310P {
    fn new(context: Context) -> Self {
        HM310P {
                context,
                features: Vec::from([
                    Feature {
                        address: 0x0000,
                        name: "ON/OFF",
                        is_rw: true,
                        quantity: 0,
                    },
                    Feature {
                        address: 0x0001,
                        //Protection status
                        name: "OP.S",
                        is_rw: false,
                        quantity: 0,
                    },
                    Feature {
                        address: 0x0002,
                        name: "Specification and type",
                        is_rw: false,
                        quantity: 0,
                    },
                    Feature {
                        address: 0x0003,
                        name: "Tail classification",
                        is_rw: false,
                        quantity: 0,
                    },
                    Feature {
                        address: 0x0004,
                        name: "Decimal point digit capacity",
                        is_rw: false,
                        quantity: 0,
                    },
                    Feature {
                        address: 0x0005,
                        name: "U",
                        is_rw: false,
                        quantity: 2,
                    },
                    Feature {
                        address: 0x0006,
                        name: "I",
                        is_rw: false,
                        quantity: 2,
                    },
                    Feature {
                        address: 0x0007,
                        name: "P",
                        is_rw: false,
                        quantity: 3,
                    },
                    Feature {
                        address: 0x0008,
                        name: "SetU",
                        is_rw: true,
                        quantity: 2,
                    },
                    Feature {
                        address: 0x0009,
                        name: "SetI",
                        is_rw: true,
                        quantity: 2,
                    },
                    Feature {
                        address: 0x000A,
                        name: "OVP",
                        is_rw: true,
                        quantity: 2,
                    },
                    Feature {
                        address: 0x000B,
                        name: "OCP",
                        is_rw: true,
                        quantity: 3,
                    },
                    Feature {
                        address: 0x000C,
                        name: "OPP",
                        is_rw: true,
                        quantity: 2,
                    },
                    Feature {
                        address: 0x000D,
                        name: "RS-Adder",
                        is_rw: true,
                        quantity: 0,
                    },
                ])
        }
    }

    fn read(&mut self, command: &Command) -> Result<Vec<u16>, std::io::Error> {
        psu::read(&mut self.context, command)
    }

    fn write(&mut self, command: &crate::psu::Command) -> std::result::Result<(), std::io::Error> {
        psu::write(&mut self.context, command)
    }

    fn features(&self) -> &Vec<Feature> {
        &self.features
    }


}

// START OF MODEL-SPECIFIC CODE
use bitflags::bitflags;

pub struct ProtectionStatus {
    flags: ProtectionStatusFlags,
    U_I_W_decimals: u8 // Number of decimals in the returned values for Voltage, Current and Power. 4 bits increments
}

bitflags! {
    struct ProtectionStatusFlags: u8 {
        const OVP =  0b0000001; // Over voltage protection
        const OCP =  0b0000010; // Over current
        const OPP =  0b0000100; // Over power protection
        const OTP =  0b0001000; // Over temperature protection
        const SCP =  0b0010000; // Short circuit protection
    }
}

pub fn parse_ProtectionStatus(response: u16) -> ProtectionStatus {
    let flags = response as u8;
    let U_I_W_decimals = (response >> 8) as u8;
    
    ProtectionStatus {
        flags: ProtectionStatusFlags::from_bits(flags).unwrap(),
        U_I_W_decimals
    }
}

impl HM310P {
    pub fn get_protection_status(&mut self) -> ProtectionStatus {
        let command: Command = Command {
            feature: self.get_feature("OP.S").unwrap(),
            parameters: 0,
        };
        let response = *self.read(&command).unwrap().get(0).unwrap();
        parse_ProtectionStatus(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn protection_status_parsing_is_correct_against_example() {
        let example: u16 = 0b0000000000110101;
        let protection_status = parse_ProtectionStatus(example);
        let flags = protection_status.flags;
        flags.contains(ProtectionStatusFlags::OVP);
        flags.contains(ProtectionStatusFlags::OPP);
        flags.contains(ProtectionStatusFlags::SCP);        
    }
}
