use dylan;
use eframe::egui;
use egui::Ui;

//TODO: Implementation needed
pub enum State {
    Open,
    Close,
}
pub struct FilterWindow {
    state: State,
}

impl FilterWindow {
    pub fn get_state(&self) -> bool {
        match &self.state {
            State::Open => true,
            State::Close => false,
        }
    }
}
