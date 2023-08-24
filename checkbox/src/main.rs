use iced::widget::{checkbox, column, container};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings, Theme};

fn main() -> iced::Result {
    CheckBox::run(Settings::default())
}

#[derive(Default)]
struct CheckBox {
    default_checkbox: bool,
    custom_checkbox: bool,
}

#[derive(Debug, Clone, Copy)]
// Control events on the screen with message?
enum Message {
    DefaultChecked(bool),
    CustomChecked(bool),
}

// Study the difference between Application and Sandbox
// Application-api  : https://docs.iced.rs/iced/application/trait.Application.html
// Sandbox-api      : https://docs.iced.rs/iced/trait.Sandbox.html
impl Application for CheckBox {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        println!("call new func");
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        println!("call title func");
        String::from("Checkbox - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        println!("call update func");
        match message {
            Message::DefaultChecked(val) => self.default_checkbox = val,
            Message::CustomChecked(val) => self.custom_checkbox = val,
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        println!("call view func");
        let default_checkbox = checkbox("default", self.default_checkbox, Message::DefaultChecked);
        let custom_checkbox = checkbox("custom", self.custom_checkbox, Message::CustomChecked);

        let content = column![default_checkbox, custom_checkbox].spacing(22);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
