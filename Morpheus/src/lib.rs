mod builder;
mod errors;
mod version;

pub use builder::*;
pub use errors::*;
pub use version::*;

use quicksand::{RegisterType, REGISTER_SRX};

pub const fn srx(x: u8) -> Result<u8> {
    if x < 6 {
        Ok(REGISTER_SRX | x)
    } else {
        Err(Error::InvalidRegisterIndex)
    }
}

pub const fn gpr(reg_type: RegisterType, x: u8) -> Result<u8> {
    if x < 32 {
        Ok(reg_type as u8 | x)
    } else {
        Err(Error::InvalidRegisterIndex)
    }
}

#[cfg(test)]
mod tests {
    use quicksand::{
        REGISTER_SR0, REGISTER_SR1, REGISTER_SR2, REGISTER_SR3, REGISTER_SR4, REGISTER_SR5,
    };

    use super::*;

    #[test]
    fn sr0() {
        let result = srx(0);
        assert!(matches!(result, Ok(REGISTER_SR0)));
    }

    #[test]
    fn sr1() {
        let result = srx(1);
        assert!(matches!(result, Ok(REGISTER_SR1)));
    }

    #[test]
    fn sr2() {
        let result = srx(2);
        assert!(matches!(result, Ok(REGISTER_SR2)));
    }

    #[test]
    fn sr3() {
        let result = srx(3);
        assert!(matches!(result, Ok(REGISTER_SR3)));
    }

    #[test]
    fn sr4() {
        let result = srx(4);
        assert!(matches!(result, Ok(REGISTER_SR4)));
    }

    #[test]
    fn sr5() {
        let result = srx(5);
        assert!(matches!(result, Ok(REGISTER_SR5)));
    }

    #[test]
    fn sr6() {
        let result = srx(6);
        assert!(matches!(result, Err(Error::InvalidRegisterIndex)));
    }

    #[test]
    fn br0() {
        let result = gpr(RegisterType::B, 0);
        assert!(matches!(result, Ok(0x40)));
    }

    #[test]
    fn br31() {
        let result = gpr(RegisterType::B, 31);
        assert!(matches!(result, Ok(0x5F)));
    }
}
