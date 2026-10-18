use super::matrix::Matrix;
use macroquad::prelude::*;

/// Iterator returning grid positions ((ix, iy), (x, y))
fn iter_grid(
    dim_x: usize,
    dim_y: usize,
    w_cell: f32,
    h_cell: f32,
) -> impl Iterator<Item = ((usize, usize), (f32, f32))> {
    gen move {
        for iy in 0..dim_y {
            for ix in 0..dim_x {
                let x = (ix as f32 * w_cell) as f32;
                let y = (iy as f32 * h_cell) as f32;
                yield ((ix, iy), (x, y))
            }
        }
    }
}

pub fn draw_grid(map: &Matrix<bool>) {
    let w_cell = screen_width() / map.width() as f32;
    let h_cell = screen_height() / map.height() as f32;

    let it = iter_grid(map.width(), map.height(), w_cell, h_cell);
    for ((ix, iy), (x, y)) in it {
        let color = if map[(iy, ix)] == true { RED } else { GREEN };

        draw_rectangle(x, y, w_cell, h_cell, color);
        draw_rectangle_lines(x, y, w_cell, h_cell, 2., BLACK);
    }
}
