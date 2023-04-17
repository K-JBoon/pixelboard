use crate::RGBMatrix;

pub trait Visualizer {
    fn new(width: usize, height: usize) -> Self;
    fn render(&mut self, matrix: RGBMatrix);
}
