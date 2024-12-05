use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub mod input;
use crate::input::{CharGrid, Config};
pub mod render;
use crate::render::Renderer;
pub mod output;
use crate::output::write_pdf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Files to render
    #[arg(required = true)]
    input: Vec<String>,

    /// Config file
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,

    /// Render grid in all board
    #[arg(short, long, default_value_t = false)]
    grid: bool,

    /// Render board frame
    #[arg(short, long, default_value_t = false)]
    frame: bool,
}

fn main() {
    let cli = Cli::parse();

    let config_path = match &cli.config {
        Some(path) => path.clone(),
        _ => Path::new(env!("CARGO_MANIFEST_DIR")).join("config.jsonc"),
    };

    let mut conf = Config::load_config(&config_path);
    conf.grid = cli.grid;
    conf.frame = cli.frame;
    let render = Renderer::new(conf.clone());

    for input_file in &cli.input {
        let grid: CharGrid = fs::read_to_string(input_file)
            .expect("Error: can't read file")
            .into();
        let doc = render.draw_grid(&grid);
        let input_path = Path::new(input_file);
        let output_filename = input_path.with_extension("pdf");
        write_pdf(&doc, output_filename.to_str().unwrap()).expect("No he pogut escrire el pdf");
    }
}
