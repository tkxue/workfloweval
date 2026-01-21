use super::*;

pub trait T_BigEnum_: Sized {
    const NUM_ARMS: u16;
    const names: &'static [&'static str];

    fn to_u16_(&self) -> U16_BigEnum<Self>;

    fn from_u16_(x: u16) -> Option<Self>;
}

pub trait T_BigEnum: T_BigEnum_ {
    fn to_u16(&self) -> U16_BigEnum<Self> {
        self.to_u16_()
    }

    fn get_name(&self) -> &'static str {
        Self::names[self.to_u16().inner as usize]
    }

    fn from_u16(x: u16) -> Option<Self> {
        Self::from_u16_(x)
    }

    fn get_num_arms() -> u16 {
        Self::NUM_ARMS
    }

    fn iter() -> impl Iterator<Item = Self> {
        (0..Self::NUM_ARMS).map(|x| Self::from_u16_(x).unwrap())
    }
}
