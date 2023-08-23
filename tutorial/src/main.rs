use iced::Settings;
use iced::pure::Sandbox;
use iced::pure::widget::{Button, Text, Column, Container};

use main_page::MainPage;

mod main_page;

#[derive(Debug, Clone, Copy)]
pub enum Views {
    Counter,
    Main
}

struct Counter {
    count: i32,
    current_view: Views,
    main_page: MainPage
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Increment,
    Decrement,
    ChangePage(Views)
}

impl Sandbox for Counter {
    type Message = CounterMessage;
    
    fn new() -> Self {
        Counter{ 
	    count: 0,
	    current_view: Views::Counter,
	    main_page: MainPage::new()
	}
    }

    fn title(&self) -> String { 
        String::from("Counter app") 
    }
  
    fn update(&mut self, message: Self::Message) { 
	match message {
	    CounterMessage::Increment => self.count += 1,
	    CounterMessage::Decrement => self.count -= 1,
	    CounterMessage::ChangePage(view) => self.current_view = view
	}
    }
    
    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        let label = Text::new(format!("Coun: {}", self.count));

	let incr = Button::new("Increment").on_press(CounterMessage::Increment);
	let decr = Button::new("Decrement").on_press(CounterMessage::Decrement);
	let navigate = Button::new(Text::new("Go to the next page")).on_press(CounterMessage::ChangePage(Views::Main));

	// let col = Column::new().push(incr).push(label).push(decr);
	let col = Column::new().push(incr).push(label).push(decr).push(navigate).spacing(5);	
	
	let counter_layout = Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill);

	// Container::new(col).center_x().center_y().width(iced::Length::Fill).height(iced::Length::Fill).into()

	match self.current_view {
	    Views::Counter => counter_layout.into(),
	    Views::Main => self.main_page.view()
	}

    }
}

fn main() -> Result<(), iced::Error> {
    Counter::run(Settings::default())
}
