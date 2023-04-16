use std::ops::{Index, IndexMut};

pub type RGB = Option<(u8, u8, u8)>;

pub struct RGBMatrix {
    pub width: usize,
    pub height: usize,
    pub matrix: Vec<RGB>,
}

impl RGBMatrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            matrix: vec![None; width * height],
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> RGB {
        let idx = row * self.width + col;

        if let Some(v) = self.matrix.get(idx) {
            *v
        } else {
            None
        }
    }

    pub fn merge(&mut self, matrix_to_merge: RGBMatrix, row_offset: usize, col_offset: usize) {
        for i in 0..matrix_to_merge.height {
            for j in 0..matrix_to_merge.width {
                let row = i + row_offset;
                let col = j + col_offset;

                if row < self.height && col < self.width {
                    let rgb = matrix_to_merge.get(i, j);
                    if rgb.is_some() {
                        self[row][col] = rgb;
                    }
                }
            }
        }
    }

    pub fn draw_border(&mut self, color: RGB) {
        let width = self.width;
        let height = self.height;

        for i in 0..width {
            self[0][i] = color;
            self[height - 1][i] = color;
        }

        for i in 0..height {
            self[i][0] = color;
            self[i][width - 1] = color;
        }
    }
}

impl Index<usize> for RGBMatrix {
    type Output = [RGB];

    #[inline]
    fn index(&self, idx: usize) -> &[RGB] {
        let start_idx = idx * self.width;
        &self.matrix[start_idx..start_idx + self.width]
    }
}

impl IndexMut<usize> for RGBMatrix {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut [RGB] {
        let start_idx = idx * self.width;
        &mut self.matrix[start_idx..start_idx + self.width]
    }
}
