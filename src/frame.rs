use iced::{Color, mouse, Rectangle, Renderer, Theme};
use iced::widget::canvas::{Frame, Geometry, Path, Program};

pub struct Circle {
    pub(crate) radius: f32,
}

// Then, we implement the `Program` trait
impl Program<()> for Circle {
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}