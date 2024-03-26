use crate::body::Body;
use crate::message::Message;
use iced::{Command, Element, executor, Theme, widget};
use iced::widget::canvas::Program;
use iced_aw;
use crate::frame::Circle;
use crate::message;

#[derive(Default)]
pub struct Rgs {
    bodies: Vec<Body>,
    sim_running: bool,
}

impl iced::Application for Rgs {
    type Executor = executor::Default;
    type Message = message::Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Rgs::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Rust Gravitation Simulator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartSimulation => { self.sim_running = true }
            Message::StopSimulation => { self.sim_running = false }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let start_sim = widget::button("Запустить")
            .on_press(Message::StartSimulation);

        let stop_sim = widget::button("Остановить")
            .on_press(Message::StopSimulation);

        let body_picker = iced_aw::selection_list(&["foo", "bar", "egg"], |_, _| {});


        let body_info = widget::Column::from_vec(
            vec![
                body_picker.into(),
                widget::Row::from_vec(
                    vec![
                        widget::text("Положение:").into(),
                        widget::text("X:").into(),
                        iced_aw::number_input(0, 100, |val| {}).into(),
                        widget::text("Y:").into(),
                        iced_aw::number_input(0, 100, |val| {}).into(),
                    ]
                ).into(),
                widget::Row::from_vec(
                    vec![
                        widget::text("Скорость:").into(),
                        widget::text("X:").into(),
                        iced_aw::number_input(0, 100, |val| {}).into(),
                        widget::text("Y:").into(),
                        iced_aw::number_input(0, 100, |val| {}).into(),
                    ]
                ).into(),
                widget::Row::from_vec(
                    vec![
                        widget::text("Масса:").into(),
                        iced_aw::number_input(0, 100, |val| {}).into(),
                    ]
                ).into(),
                widget::Row::from_vec(
                    vec![
                        widget::text("Радиус:").into(),
                        iced_aw::number_input(0, 100, |val| {}).into(),
                    ]
                ).into(),
            ]
        );

        let sim_settings = widget::Column::from_vec(
            vec![

            ]
        );

        let canvas = widget::canvas(Circle {radius: 10.0 });

        let tools_column = widget::Column::from_vec(
            vec![
                widget::Container::new(body_info).into(),
            ]
        );

        let interface = widget::Row::from_vec(
            vec![
                tools_column.into(),
                canvas.into(),
            ]);

        widget::container(interface).into()
    }
}