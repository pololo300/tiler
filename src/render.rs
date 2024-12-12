use crate::CharGrid;
use svg::node::element::{Circle, Line, Rectangle};
use svg::Document;
use unicode_normalization::UnicodeNormalization;

use crate::input::Config;

pub struct Renderer {
    conf: Config,

    // to avoid loop dots painting
    first_dot: bool,
    dot_x: usize,
    dot_y: usize,
}

impl Renderer {
    pub fn new(conf: Config) -> Renderer {
        Renderer {
            conf,
            first_dot: false,
            dot_x: 0,
            dot_y: 0,
        }
    }

    pub fn draw_grid(&mut self, grid: &CharGrid) -> Document {
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
        for (x, row) in grid.grid.iter().enumerate() {
            for (y, &c) in row.iter().enumerate() {
                if c == 'X' {
                    continue;
                }

                let color = self.get_color(grid, x, y);
                let stroke = {
                    if c == '.' || c == '·' {
                        false
                    } else {
                        self.conf.grid || (color != "none" && color != "white")
                    }
                };
                doc = doc.add(self.paint_cell(x as i32, y as i32, color.as_str(), stroke));
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

                    let (x1, y1, x2, y2) = match (dx, dy) {
                        (1, 0) => (1, 0, 1, 1),
                        (-1, 0) => (0, 0, 0, 1),
                        (0, 1) => (0, 1, 1, 1),
                        (0, -1) => (0, 0, 1, 0),
                        _ => (0, 0, 0, 0),
                    };

                    if !x_border
                        && !y_border
                        && c == grid.grid[(y + dy) as usize][(x + dx) as usize]
                    {
                        if c == '·' || c == '.' {
                            doc = doc.add(self.circle(x + x1, y + y1, x + x2, y + y2));
                        }

                        continue;
                    }

                    doc = doc.add(self.border(x + x1, y + y1, x + x2, y + y2));
                }
            }

            // finalment pintem els puntets
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

    fn circle(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Circle {
        let x = ((x1 as f32 + x2 as f32) / 2.0) * self.conf.cell_size as f32;
        let y = ((y1 as f32 + y2 as f32) / 2.0) * self.conf.cell_size as f32;
        Circle::new()
            .set("cx", x as u32 + self.conf.border_width)
            .set("cy", y as u32 + self.conf.border_width)
            .set("r", 2 * self.conf.border_width / 3)
            .set("fill", "black")
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

    fn paint_cell(&self, x: i32, y: i32, color: &str, stroke: bool) -> Rectangle {
        let mut rect = Rectangle::new()
            .set(
                "y",
                x as u32 * self.conf.cell_size + self.conf.border_width - 1,
            )
            .set(
                "x",
                y as u32 * self.conf.cell_size + self.conf.border_width - 1,
            )
            .set("width", self.conf.cell_size + 1)
            .set("height", self.conf.cell_size + 1)
            .set("fill", color);

        if stroke {
            rect = rect
                .set("stroke", "black")
                .set("stroke-width", self.conf.separator_width);
        }

        rect
    }

    fn get_color(&mut self, grid: &CharGrid, x: usize, y: usize) -> String {
        let c = grid.get(x, y);
        let char_without_punctuation = c.nfd().next().unwrap_or(c);
        if c != '.' && c != '·' {
            return self
                .conf
                .colors
                .get(&char_without_punctuation)
                .unwrap()
                .to_string();
        };

        if !self.first_dot {
            self.first_dot = true;
            self.dot_x = x;
            self.dot_y = y;
        } else if x == self.dot_x && y == self.dot_y {
            panic!("Dots loop!")
        }

        let mut left = y;
        for j in (0..y).rev() {
            if c == grid.get(x, j) {
                left = j;
            } else {
                break;
            }
        }
        let mut right = y;
        for j in y + 1..grid.width {
            if c == grid.get(x, j) {
                right = j;
            } else {
                break;
            }
        }

        let mut bottom = x;
        for i in (0..x).rev() {
            if c == grid.get(i, y) {
                bottom = i;
            } else {
                break;
            }
        }
        let mut top = x;
        for i in x + 1..grid.height {
            if c == grid.get(i, y) {
                top = i;
            } else {
                break;
            }
        }

        let width = right - left + 1;
        let height = top - bottom + 1;
        if width > 1 && height > 1 {
            panic!("Dots not alliged");
        }

        let length = {
            if width > 1 {
                width
            } else {
                height
            }
        };

        if length < 3 {
            panic!("Dots two short, mimum 3");
        }

        if length == width && (left == 0 || right == grid.width - 1) {
            panic!("Dots can't end in border")
        }
        if length == height && (bottom == 0 || top == grid.height - 1) {
            panic!("Dots can't end in border")
        }

        let color1: String;
        let color2: String;
        if length == width {
            color1 = self.get_color(grid, x, left - 1);
            color2 = self.get_color(grid, x, right + 1);
        } else {
            color1 = self.get_color(grid, top + 1, y);
            color2 = self.get_color(grid, bottom - 1, y);
        }

        if color1 != color2 {
            panic!("Can't color dots, mismatched end and start colors")
        }

        color1
    }
}
