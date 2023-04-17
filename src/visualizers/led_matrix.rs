use rpi_led_panel::HardwareMapping;

use crate::visualizers::Visualizer;
use crate::RGBMatrix;

pub struct LEDMatrixVisualizer {
    width: usize,
    height: usize,
    hardware_matrix: rpi_led_panel::RGBMatrix,
    canvas: Box<rpi_led_panel::Canvas>
}

impl Visualizer for LEDMatrixVisualizer {
    fn new(width: usize, height: usize) -> Self {
        let config = rpi_led_panel::RGBMatrixConfig {
            cols: width,
            rows: height,
            slowdown: Some(4),
            hardware_mapping: HardwareMapping::regular(),
            ..Default::default()
        };

        let (matrix, canvas) = rpi_led_panel::RGBMatrix::new(config, 0).expect("Matrix initialization failed");

        Self {
            width,
            height,
            hardware_matrix: matrix,
            canvas
        }
    }

    fn render(&mut self, matrix: RGBMatrix) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(color) = matrix[y][x] {
                    self.canvas.set_pixel(x, y, color.0, color.1, color.2);
                } else {
                    self.canvas.set_pixel(x, y, 0, 0, 0);
                }
            }
        }

        self.canvas.set_brightness(50);
        self.canvas = self.hardware_matrix.update_on_vsync(self.canvas.clone());
    }
}
