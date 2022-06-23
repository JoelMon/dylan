use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FedExError {
    #[error(
        "[ {function:?} ] FedEx tracking numbers are 12 digits long, {got:?} digits were entered."
    )]
    InvalidNumber { function: &'static str, got: usize },
}

#[derive(Debug, PartialEq)]
pub struct Tracking(usize);

impl Tracking {
    pub fn add(tracking_number: usize) -> Result<Tracking, FedExError> {
        let length = tracking_number.to_string().len();

        if length == 12 {
            Ok(Tracking(tracking_number))
        } else {
            Err(FedExError::InvalidNumber {
                function: "Tracking::add()",
                got: length,
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Context;

    use super::*;
    #[test]
    fn t_tracking_add() {
        let valid_track: usize = 548745654123;

        let result = Tracking::add(valid_track).context("Error while testing Tracking::add()");

        assert_eq!(result.unwrap(), Tracking(valid_track));
    }
}
