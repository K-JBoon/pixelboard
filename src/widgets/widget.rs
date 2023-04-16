use std::time::Duration;

use crate::RGBMatrix;

// TODO: design some sort of should_render mechanism
pub trait Widget {
    fn render(&mut self, width: usize, height: usize, elapsed_time: Duration) -> RGBMatrix;
}
