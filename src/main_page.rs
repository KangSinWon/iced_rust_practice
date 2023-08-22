use iced::{pure::{
    widget::{Container, Text},
    Element,
}, Length};

use crate::CounterMessage;

pub struct MainPage;

impl MainPage {
    pub fn new() -> Self {
        MainPage
    }

    pub fn view(&self) -> Element<CounterMessage> {
        Container::new(Text::new("Hello from Page 2"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
