use crate::psu::PSU;
use crate::{
    psu::Command,
    sku::Feature,
};
use flagset::{flags, FlagSet};
use tokio_modbus::client::Context;

pub struct HM310P {
    pub psu: PSU,
}

// START OF MODEL-SPECIFIC CODE

use super::{RWCapabilities};

pub struct ProtectionStatus {
    flags: FlagSet<ProtectionStatusFlags>,
    u_i_w_decimals: u8, // Number of decimals in the returned values for Voltage, Current and Power. 4 bits increments
}

flags! {
    pub enum ProtectionStatusFlags: u8 {
        OVP = 0b0000001, // Over voltage protection
        OCP = 0b0000010, // Over current
        OPP = 0b0000100, // Over power protection
        OTP = 0b0001000, // Over temperature protection
        SCP = 0b0010000, // Short circuit protection
    }
}

pub fn parse_protection_status(response: u16) -> ProtectionStatus {
    let flags = response as u8;
    let u_i_w_decimals = (response >> 8) as u8;

    ProtectionStatus {
        flags: FlagSet::<ProtectionStatusFlags>::new(flags).unwrap(),
        u_i_w_decimals,
    }
}

impl HM310P {
    pub fn new(context: Context) -> HM310P {
        HM310P {
            psu: PSU {
                context,
                features: Vec::from([
                    Feature::new(
                        0x0000,
                        "ON/OFF",
                         RWCapabilities::Write,
                        0,
                    ),
                    Feature::new (
                        0x0001,
                        //Protection status
                         "OP.S",
                        RWCapabilities::Read,
                        0,
                    ),
                    Feature::new (
                        0x0002,
                        "Specification and type",
                        RWCapabilities::Read,
                        0,
                    ),
                    Feature::new (
                        0x0003,
                        "Tail classification",
                        RWCapabilities::Read,
                        0,
                    ),
                    Feature::new (
                        0x0004,
                        "Decimal point digit capacity",
                        RWCapabilities::Read,
                        0,
                    ),
                    Feature::new (
                        0x0005,
                        "U",
                        RWCapabilities::Read,
                        2,
                    ),
                    Feature::new (
                        0x0006,
                        "I",
                        RWCapabilities::Read,
                        2,
                    ),
                    Feature::new (
                        0x0007,
                        "P",
                        RWCapabilities::Read,
                        3,
                    ),
                    Feature::new (
                        0x0008,
                        "SetU",
                        RWCapabilities::Write,
                        2,
                    ),
                    Feature::new (
                        0x0009,
                        "SetI",
                        RWCapabilities::Write,
                        2,
                    ),
                    Feature::new (
                        0x000A,
                        "OVP",
                        RWCapabilities::Read | RWCapabilities::Write,
                        2,
                    ),
                    Feature::new (
                        0x000B,
                        "OCP",
                        RWCapabilities::Read | RWCapabilities::Write,
                        3,
                    ),
                    Feature::new (
                        0x000C,
                        "OPP",
                        RWCapabilities::Read | RWCapabilities::Write,
                        2,
                    ),
                    Feature::new (
                        0x000D,
                        "RS-Adder",
                        RWCapabilities::Read,
                        0,
                    ),
                ]),
            },
        }
    }
    pub async fn get_protection_status(&mut self) -> ProtectionStatus {
        let command: Command = Command {
            mode: RWCapabilities::Read,
            feature: self.psu.get_feature("OP.S").unwrap(),
            parameters: 0,
        };
        let response = *self.psu.read(&command).await.unwrap().get(0).unwrap();
        parse_protection_status(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn protection_status_parsing_is_correct_against_example() {
        let example: u16 = 0b0000000000110101;
        let protection_status = parse_protection_status(example);
        let flags = protection_status.flags;
        flags.contains(ProtectionStatusFlags::OVP);
        flags.contains(ProtectionStatusFlags::OPP);
        flags.contains(ProtectionStatusFlags::SCP);
    }
}
