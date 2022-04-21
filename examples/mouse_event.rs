use rustea::{
    command::quit,
    crossterm::event::{MouseEvent, MouseEventKind},
    App, Command, Message,
};

struct Model {
    col: u16,
    row: u16,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(mouse_event) = msg.downcast::<MouseEvent>() {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                return Some(Box::new(quit));
            }
            self.col = mouse_event.column;
            self.row = mouse_event.row;
        }

        None
    }

    fn view(&self) -> String {
        format!(
            "Click to terminate. Mouse row: {}, col: {}",
            self.col, self.row
        )
    }
}

fn main() {
    let model = Model { col: 0, row: 0 };

    rustea::enable_mouse_capture().unwrap();
    rustea::run(model).unwrap();
}
