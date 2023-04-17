use crate::visualizers::Visualizer;
use crate::RGBMatrix;
use termion::{color, style};

pub struct TerminalVisualizer {
    width: usize,
    height: usize,
}

impl Visualizer for TerminalVisualizer {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    fn render(&mut self, matrix: RGBMatrix) {
        let mut output = format!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::AfterCursor
        );

        for i in 0..self.height {
            for j in 0..self.width {
                if let Some((r, g, b)) = matrix[i][j] {
                    output += format!("{}", color::Fg(color::Rgb(r, g, b))).as_str();
                }

                output += "██";
                output += format!("{}", style::Reset).as_str();
            }
            output += "\n"
        }

        println!("{}", output);
    }
}
