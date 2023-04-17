use std::time::Duration;
use chrono::prelude::*;
use crate::{RGB, RGBMatrix, Widget, TextWidget};

pub struct ClockWidget {
    text_widget: TextWidget
}

// TODO: numbers need custom rasterization or a different font
impl ClockWidget {
    pub fn new(color: RGB) -> Self {
        Self {
            text_widget: TextWidget::new("00:00".to_owned(), color)
        }
    }
}

impl Widget for ClockWidget {
    fn render(&mut self, width: usize, height: usize, _elapsed_time: Duration) -> RGBMatrix {
        let localtime: DateTime<Local> = Local::now();
        self.text_widget.update_text(format!("{} {}", localtime.hour() ,localtime.minute()));

        self.text_widget.render(width, height, _elapsed_time)
    }
}
