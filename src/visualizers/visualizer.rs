use crate::RGBMatrix;

pub trait Visualizer {
    fn new(width: usize, height: usize) -> Self;
    fn render(&self, matrix: RGBMatrix);
}
