use std::{collections::HashMap, time::Duration};
use crate::{RGBMatrix, Widget, RGB};
use fontdue::{Font, Metrics};

const FONT_DATA: &[u8] = include_bytes!("../../assets/standard.ttf");

pub struct TextWidget {
    text: String,
    color: (f32, f32, f32),
    font: Font,
    total_width: usize,
    tallest_character_height: usize,
    last_height: usize,
    last_width: usize,
    rasters: HashMap<char, (Metrics, Vec<u8>)>
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
            tallest_character_height: 0,
            last_height: 0,
            last_width: 0,
            rasters: HashMap::new()
        }
    }

    fn rasterize_characters(&mut self, width: usize, height: usize) {
        if self.last_width != width || self.last_height != height {
            self.last_height = height;
            self.last_width = width;
            self.total_width = 0;
            self.tallest_character_height = 0;
            self.rasters.clear();

            for char in self.text.chars() {
                if !self.rasters.contains_key(&char) {
                    let (metrics, bitmap) = self.font.rasterize(char, height as f32 - 4.0);
                    self.rasters.insert(char, (metrics, bitmap));
                }

                if let Some((metrics, _)) = self.rasters.get(&char) {
                    self.total_width += metrics.width + 1;

                    if self.tallest_character_height < metrics.height {
                        self.tallest_character_height = metrics.height;
                    }
                }
            }
        }

        if self.total_width > width {
            self.rasterize_characters(width, height - 1);
        }
    }
}

impl Widget for TextWidget {
    fn render(&mut self, width: usize, height: usize, _elapsed_time: Duration) -> RGBMatrix {
        // This will re-rasterize if the dimensions have changed, and otherwise use already
        // computed rasters
        self.rasterize_characters(width, height);

        let mut matrix = RGBMatrix::new(width, height);

        // Horizontally center the text
        let mut col_offset = (width - self.total_width) / 2;

        let target_bottom_offset = (height - self.tallest_character_height) / 2;

        for char in self.text.chars() {
            let (metrics, bitmap) = self.rasters.get(&char).expect("Did not have rasters for the given char");

            let row_offset = height - metrics.height - target_bottom_offset;

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let char_s = bitmap[x + y * metrics.width] as f32;

                    // TODO: scale minimum coverage by font size
                    if char_s > 150_f32 {
                        let col = col_offset + x;
                        let row = row_offset + y;

                        if col > 0 && col < matrix.width && row > 0 && row < matrix.height {
                            let c0 = self.color.0 as f32 * (char_s / 255.0);
                            let c1 = self.color.1 as f32 * (char_s / 255.0);
                            let c2 = self.color.2 as f32 * (char_s / 255.0);
                            matrix[row][col] = Some((c0 as u8, c1 as u8, c2 as u8));
                        }
                    }
                }
            }

            col_offset += metrics.width + 1;
        }

        matrix
    }
}
