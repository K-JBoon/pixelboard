use std::time::Duration;

use crate::{RGBMatrix, Widget, RGB};

pub struct SolidBlockWidget {
    color: RGB,
}

impl SolidBlockWidget {
    pub fn new(color: RGB) -> Self {
        Self { color }
    }
}

impl Widget for SolidBlockWidget {
    fn render(&mut self, width: usize, height: usize, _elapsed_time: Duration) -> RGBMatrix {
        // TODO: no need to reconstruct this matrix in a hot loop
        RGBMatrix {
            width,
            height,
            matrix: vec![self.color; width * height],
        }
    }
}
