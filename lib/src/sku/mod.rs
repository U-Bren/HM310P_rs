use self::hm310p::HM310P;

pub mod hm310p;

#[derive(Debug, Clone, Copy)]
pub struct Feature {
    pub address: u16,
    pub name: &'static str,
    pub is_rw: bool,
    pub quantity: u16, //number of bytes taken as arg
}