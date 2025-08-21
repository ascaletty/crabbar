use iced::Length::Fill;
use iced::Size;
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::widget::{Column, Container, Text, center, container, row, text, text_input};
use iced::{Color, Element, Event, Length, Task as Command, Theme, event, widget::Row};
use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;
use std::ffi::c_long;
use std::process::Command as ExecCommand;

struct Counter {
    value: i32,
    text: String,
}

#[derive(Debug, Clone, Copy)]
enum WindowDirection {
    Top,
    Left,
    Right,
    Bottom,
}

// Because new iced delete the custom command, so now we make a macro crate to generate
// the Command
#[to_layer_message]
#[derive(Debug, Clone)]
#[doc = "Some docs"]
enum Message {
    WorkspaceChanged(i32),
    TextInput(String),
    IcedEvent(Event),
    Enter(String),
    FontLoaded(Result<(), iced::font::Error>),
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                text: "".to_string(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Counter - Iced")
    }
    //
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IcedEvent(event) => {
                println!("hello {event:?}");
                Command::none()
            }
            Message::WorkspaceChanged(workspace_id) => {
                self.value += 1;
                print!("switched to workspace_id {workspace_id}");
                Command::none()
            }
            Message::TextInput(text) => {
                self.text = text;
                Command::none()
            }
            Message::Enter(text) => {
                self.text = "".to_string();
                let mut result = ExecCommand::new(&text).spawn();
                match result {
                    Ok(output) => println!("Success {:?}", output),
                    Err(e) => eprintln!("Failed to run command: {}", e),
                }
                Command::none()
            }

            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let clock = text("11:11ffcsbcbkjsdbjcsbkdcsjcebcjekdsjkj");
        // .width(Length::Shrink.enclose(Length::Fixed(250.0)))
        // .padding(1)
        // .style(|theme| container::Style {
        //     background: Some(Theme::CatppuccinMocha.palette().background.into()),
        //     border: iced::Border {
        //         color: Theme::CatppuccinMocha
        //             .extended_palette()
        //             .secondary
        //             .base
        //             .color,
        //         width: 2.0,
        //         radius: Radius::new(10),
        //     },
        //     ..Default::default()
        // });
        // .align_x(Horizontal::Left)
        let drun = text_input("drun", &self.text)
            .on_input(Message::TextInput)
            .on_submit(Message::Enter(self.text.clone()))
            .align_x(Horizontal::Center)
            .width(Length::Fixed(250.0))
            .style(|_theme, _| text_input::Style {
                background: Theme::CatppuccinMocha.palette().background.into(),
                icon: Theme::CatppuccinMocha
                    .extended_palette()
                    .background
                    .base
                    .color,
                placeholder: Theme::CatppuccinMocha
                    .extended_palette()
                    .background
                    .strong
                    .color,
                selection: Theme::CatppuccinMocha
                    .extended_palette()
                    .background
                    .strong
                    .color,
                value: Theme::CatppuccinMocha
                    .extended_palette()
                    .secondary
                    .base
                    .text,
                border: iced::Border {
                    color: Theme::CatppuccinMocha
                        .extended_palette()
                        .background
                        .base
                        .color,
                    width: 0.1,
                    radius: Radius::new(10),
                },
            });

        row![clock, drun].spacing(200).into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}
pub fn main() -> Result<(), iced_layershell::Error> {
    // let myfont= iced::font::load(include_bytes!("../assets/Hack.ttf")).collect()
    let binded_output_name = std::env::args().nth(1);
    let start_m = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };
    // let mut font_vec: Vec<Cow<[u8]>> = vec![];
    // font_vec.push(
    //     include_bytes!("fonts/FiraCode-Regular.ttf")
    //         .as_slice()
    //         .into(),
    // );
    // MainSettings {
    //     fonts: font_vec,
    //     ..Default::default()
    // };
    Counter::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 30)),
            exclusive_zone: 30,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            start_mode: start_m,
            events_transparent: false,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            layer: iced_layershell::reexport::Layer::Overlay,
            margin: (20, 20, 20, 20),
        },

        ..Default::default()
    })
}
