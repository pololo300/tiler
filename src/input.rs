#[derive(Debug)]
pub struct CharGrid {
    pub grid: Vec<Vec<char>>, // 2D vector to hold the grid of characters
    pub width: usize,         // Width of the grid
    pub height: usize,        // Height of the grid
}

impl CharGrid {
    fn _cells(&self) -> impl Iterator<Item = &char> {
        self.grid.iter().flat_map(|row| row.iter())
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.grid[x][y]
    }
}

use json_comments::StripComments;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

impl From<String> for CharGrid {
    fn from(input: String) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();

        // Determine the maximum width based on the longest line
        let width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        // Initialize the grid, filling missing spaces
        let grid = lines
            .iter()
            .map(|line| {
                let mut row: Vec<char> = line.chars().collect();
                row.resize(width, ' '); // Fill with spaces to ensure consistent width
                row
            })
            .collect();

        CharGrid {
            grid,
            width,
            height,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub cell_size: u32,
    pub border_width: u32,
    pub separator_width: u32,
    pub frame: bool,
    pub grid: bool,
    pub colors: HashMap<char, String>,
}

impl Config {
    pub fn load_config(file_path: &PathBuf) -> Config {
        let file = File::open(file_path).expect("Failed to open the config file");
        let mut stripped_content = String::new();
        let mut reader = StripComments::new(file);
        reader
            .read_to_string(&mut stripped_content)
            .expect("Failed to read and strip comments from the JSONC file");
        serde_json::from_str(&stripped_content).expect("Failed to parse the JSON config")
    }
}
