use quicksand::REGISTER_SRX;

pub enum Error {
    SRXOutOfBounds,
}

pub const fn srx(x: u8) -> Result<u8, Error> {
    if x < 6 {
        Ok(REGISTER_SRX | x)
    } else {
        Err(Error::SRXOutOfBounds)
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
        assert!(matches!(result, Err(Error::SRXOutOfBounds)));
    }
}
