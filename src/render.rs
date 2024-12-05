use crate::CharGrid;
use svg::node::element::{Line, Rectangle};
use svg::Document;
use unicode_normalization::UnicodeNormalization;

use crate::input::Config;

pub struct Renderer {
    conf: Config,
}

impl Renderer {
    pub fn new(conf: Config) -> Renderer {
        Renderer { conf }
    }

    pub fn draw_grid(&self, grid: &CharGrid) -> Document {
        let mut doc = Document::new().set(
            "viewBox",
            (
                0,
                0,
                grid.width as u32 * self.conf.cell_size + 2 * self.conf.border_width,
                grid.height as u32 * self.conf.cell_size + 2 * self.conf.border_width,
            ),
        );

        // primer pintem les caselles
        for (y, row) in grid.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == 'X' {
                    continue;
                }

                doc = doc.add(self.paint_cell(x as i32, y as i32, c));
            }
        }

        // despres pintem els marges
        for (y, row) in grid.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == 'X' || c == ' ' {
                    continue;
                }
                let difs_x: Vec<i32> = vec![1, 0, -1, 0];
                let difs_y: Vec<i32> = vec![0, 1, 0, -1];

                for dir in 0..4 {
                    let x: i32 = x as i32;
                    let y: i32 = y as i32;
                    let dx: i32 = difs_x[dir];
                    let dy: i32 = difs_y[dir];

                    let x_border =
                        (x == 0 && dx == -1) || (x as usize == grid.width - 1 && dx == 1);
                    let y_border =
                        (y == 0 && dy == -1) || (y as usize == grid.height - 1 && dy == 1);

                    if !x_border
                        && !y_border
                        && c == grid.grid[(y + dy) as usize][(x + dx) as usize]
                    {
                        continue;
                    }

                    let (x1, y1, x2, y2) = match (dx, dy) {
                        (1, 0) => (1, 0, 1, 1),
                        (-1, 0) => (0, 0, 0, 1),
                        (0, 1) => (0, 1, 1, 1),
                        (0, -1) => (0, 0, 1, 0),
                        _ => (0, 0, 0, 0),
                    };

                    doc = doc.add(self.border(x + x1, y + y1, x + x2, y + y2));
                }
            }
        }

        // border frame
        if self.conf.frame {
            doc = doc
                .add(self.border(0, 0, 0, grid.height as i32))
                .add(self.border(0, grid.height as i32, grid.width as i32, grid.height as i32))
                .add(self.border(grid.width as i32, 0, grid.width as i32, grid.height as i32));
        }

        doc
    }

    fn border(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        Line::new()
            .set(
                "x1",
                x1 as u32 * self.conf.cell_size + self.conf.border_width,
            )
            .set(
                "y1",
                y1 as u32 * self.conf.cell_size + self.conf.border_width,
            )
            .set(
                "x2",
                x2 as u32 * self.conf.cell_size + self.conf.border_width,
            )
            .set(
                "y2",
                y2 as u32 * self.conf.cell_size + self.conf.border_width,
            )
            .set("stroke", "black")
            .set("stroke-linecap", "round")
            .set("stroke-width", self.conf.border_width)
    }

    fn paint_cell(&self, x: i32, y: i32, c: char) -> Rectangle {
        let without_accent = c.nfd().next().unwrap_or(c);
        let color = self.conf.colors.get(&without_accent).unwrap().as_str();
        let mut rect = Rectangle::new()
            .set("x", x as u32 * self.conf.cell_size + self.conf.border_width)
            .set("y", y as u32 * self.conf.cell_size + self.conf.border_width)
            .set("width", self.conf.cell_size)
            .set("height", self.conf.cell_size)
            .set("fill", color);
        if self.conf.grid || (color != "none" && color != "white") {
            rect = rect
                .set("stroke", "black")
                .set("stroke-width", self.conf.separator_width);
        }

        rect
    }
}
