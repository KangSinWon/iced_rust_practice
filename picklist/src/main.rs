use iced::widget::{container, pick_list, scrollable, vertical_space, Column, Text};
use iced::{Alignment, Element, Length, Sandbox, Settings};

use std::fmt::Formatter;

fn main() -> iced::Result {
    Picklist::run(Settings::default())
}

#[derive(Default)]
struct Picklist {
    selected_language: Option<Language>,
}

#[derive(Debug, Clone, Copy)]
enum PicklistMessage {
    LanguageSelected(Language),
}

impl Sandbox for Picklist {
    type Message = PicklistMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("pick list")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            PicklistMessage::LanguageSelected(lang) => {
                self.selected_language = Some(lang);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let pick_list = pick_list(
            &Language::ALL[..],
            self.selected_language,
            PicklistMessage::LanguageSelected,
        )
        .placeholder("Choose a language...");

        let label = match self.selected_language {
            None => Text::new(format!("Which is your favorite language?")),
            Some(lang) => Text::new(format!("Which is your favorite language? {}", lang)),
        };

        let content = Column::new()
            .push(label)
            .push(pick_list)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Rust,
    Elm,
    Ruby,
    Cpp,
    Javascript,
    Other,
}

impl Language {
    const ALL: [Language; 6] = [
        Language::Rust,
        Language::Elm,
        Language::Ruby,
        Language::Cpp,
        Language::Javascript,
        Language::Other,
    ];
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Elm => "Elm",
                Language::Ruby => "Ruby",
                Language::Cpp => "Cpp",
                Language::Javascript => "Javascript",
                Language::Other => "???",
            }
        )
    }
}
