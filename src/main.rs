use std::env;
use std::fs;
use std::path::Path;

pub mod input;
use crate::input::{CharGrid, Config};
pub mod render;
use crate::render::Renderer;
pub mod output;
use crate::output::write_pdf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut conf = Config::load_config("/home/pol/tfg/tiler/config.jsonc");
    let grid: CharGrid = fs::read_to_string(&args[1])
        .expect("Error: can't read file")
        .into();

    if (args.len() == 3) && (args[2] == "-f") {
        conf.frame = true
    }

    let render = Renderer::new(conf);

    let doc = render.draw_grid(&grid);

    let input_path = Path::new(&args[1]);
    let output_filename = input_path.with_extension("pdf");
    write_pdf(&doc, output_filename.to_str().unwrap()).expect("No he pogut escrire el pdf");
}
