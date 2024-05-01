use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() {
    Counter::run(Settings::default());
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter::default()
    }

    fn title(&self) -> String {
        String::from("Counter Example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::Increment),
            )
            .push(
                Text::new(format!("Counter: {}", self.value))
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::Decrement),
            )
            .into()
    }
}
