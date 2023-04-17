use std::time::Duration;

use crate::{RGBMatrix, Widget, RGB};

pub struct SolidBlockWidget {
    color: RGB,
    border: RGB,
}

impl SolidBlockWidget {
    pub fn new(color: RGB, border: RGB) -> Self {
        Self { color, border }
    }
}

impl Widget for SolidBlockWidget {
    fn render(&mut self, width: usize, height: usize, _elapsed_time: Duration) -> RGBMatrix {
        // TODO: no need to reconstruct this matrix in a hot loop
        let mut matrix = RGBMatrix {
            width,
            height,
            matrix: vec![self.color; width * height],
        };

        if self.border.is_some() {
            matrix.draw_border(self.border);
        }

        matrix
    }
}
