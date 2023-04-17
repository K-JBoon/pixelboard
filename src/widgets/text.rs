use std::{collections::HashMap, time::Duration};
use crate::{RGBMatrix, Widget, RGB};
use fontdue::{Font, Metrics};

const FONT_DATA: &[u8] = include_bytes!("../../assets/standard.ttf");

pub struct TextWidget {
    text: String,
    color: (f32, f32, f32),
    font: Font,
    total_width: usize,
    last_height: usize,
    rasters: HashMap<char, (Metrics, Vec<u8>)>,
    scroll_position: f32,
    scroll_speed: usize
}

impl TextWidget {
    pub fn new(text: String, color: RGB) -> Self {
        Self {
            text,
            color: if let Some(c) = color { 
                (c.0 as f32, c.1 as f32, c.2 as f32)
            } else { (0.0, 0.0, 0.0) },
            font: Font::from_bytes(FONT_DATA, fontdue::FontSettings::default()).unwrap(),
            total_width: 0,
            last_height: 0,
            rasters: HashMap::new(),
            scroll_position: 0.0,
            scroll_speed: 50 // TODO: make configurable
        }
    }

    fn rasterize_characters(&mut self, height: usize) {
        if self.last_height != height {
            self.last_height = height;
            self.total_width = 0;
            self.rasters.clear();

            for char in self.text.chars() {
                if let Some((metrics, _)) = self.rasters.get(&char) {
                    self.total_width += metrics.width + 1;
                } else {
                    let (metrics, bitmap) = self.font.rasterize(char, height as f32);
                    self.rasters.insert(char, (metrics, bitmap));

                    self.total_width += metrics.width + 1;
                }
            }
        }
    }
}

impl Widget for TextWidget {
    fn render(&mut self, width: usize, height: usize, elapsed_time: Duration) -> RGBMatrix {
        self.rasterize_characters(height);

        if width < self.total_width {
            self.scroll_position += self.scroll_speed as f32 * elapsed_time.as_secs_f32();

            if self.scroll_position > self.total_width as f32 {
                self.scroll_position = 0.0;
            }
        } else {
            self.scroll_position = 0.0;
        }

        let mut col_offset = self.scroll_position as usize;
        let mut matrix = RGBMatrix::new(width, height);

        for char in self.text.chars() {
            let (metrics, bitmap) = self.rasters.get(&char).expect("Did not have rasters for the given char");

            let row_offset = if metrics.height >= matrix.height { 0 } else { matrix.height - metrics.height };
            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let char_s = bitmap[x + y * metrics.width] as f32;

                    // TODO: scale minimum coverage by font size
                    if char_s > 125_f32 {
                        let col = col_offset + x;
                        let row = row_offset + y;

                        if col > 0 && col < matrix.width && row > 0 && row < matrix.height {
                            let c0 = self.color.0 as f32 * (char_s / 255.0);
                            let c1 = self.color.1 as f32 * (char_s / 255.0);
                            let c2 = self.color.2 as f32 * (char_s / 255.0);
                            matrix[row][col] = Some((c0 as u8, c1 as u8, c2 as u8));
                        } else if col > width {
                            let wrapped_col = col_offset + x - width;

                            if wrapped_col < matrix.width && row > 0 && row < matrix.height {
                                let c0 = self.color.0 as f32 * (char_s / 255.0);
                                let c1 = self.color.1 as f32 * (char_s / 255.0);
                                let c2 = self.color.2 as f32 * (char_s / 255.0);
                                matrix[row][wrapped_col] = Some((c0 as u8, c1 as u8, c2 as u8));

                            }
                        }
                    }
                }
            }

            col_offset += metrics.width + 1;
        }

        matrix
    }
}
