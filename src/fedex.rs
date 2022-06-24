use thiserror::Error;

#[derive(Error, Debug)]
pub enum FedExError {
    #[error(
        "[ fedex::{function:?} ] FedEx tracking numbers are 12 digits long, {got:?} digits were entered."
    )]
    InvalidNumber { function: &'static str, got: usize },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tracking(usize);

#[allow(unused)]
impl Tracking {
    /// Returns a Tracking type
    pub fn from(tracking_number: usize) -> Result<Tracking, FedExError> {
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

    /// Returns the tracking number as a `usize`.
    pub fn as_usize(self) -> usize {
        self.0.to_owned()
    }

    /// Returns the tracking numer as a `String`.
    pub fn as_string(self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Context;

    use super::*;

    // Test a valid 12 digit Fedex tracking number
    #[test]
    fn t_tracking_add_valid() {
        let valid_track: usize = 548745654123;
        let result = Tracking::from(valid_track).context("Error while testing Tracking::add()");
        assert_eq!(result.unwrap(), Tracking(valid_track));
    }

    // Test a invalid 8 digit Fedex tracking number
    #[test]
    #[should_panic = "Error while testing Tracking::add()"]
    fn t_tracking_add_under() {
        let tracking_number: usize = 55654123;
        let result = Tracking::from(tracking_number).context("Error while testing Tracking::add()");
        assert_eq!(result.unwrap(), Tracking(tracking_number));
    }

    // Test a invalid 13 digit Fedex tracking number
    #[test]
    #[should_panic = "Error while testing Tracking::add()"]
    fn t_tracking_add_over() {
        let tracking_number: usize = 5565414939230;
        let result = Tracking::from(tracking_number).context("Error while testing Tracking::add()");
        assert_eq!(result.unwrap(), Tracking(tracking_number));
    }

    // Test converting tracking number to a `usize`
    #[test]
    fn t_tracking_as_usize() {
        let tracking_number: usize = 556541493920;

        let f: Tracking = Tracking::from(tracking_number)
            .expect("Something went wrong in converting usize to Tracking");
        let result: usize = f.as_usize();
        assert_eq!(result, tracking_number);
    }

    // Test converting tracking number to a `String`
    #[test]
    fn t_tracking_as_string() {
        let tracking_number: usize = 556541493920;

        let f: Tracking = Tracking::from(tracking_number)
            .expect("Something went wrong in converting usize to Tracking");
        let result: String = f.as_string();
        assert_eq!(result, tracking_number.to_string());
    }
}
