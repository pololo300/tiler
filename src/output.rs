use serde_json::Result;
use svg::Document;
use svg2pdf::usvg::fontdb;
use svg2pdf::{ConversionOptions, PageOptions};

pub fn write_pdf(svg: &Document, out_path: &str) -> Result<()> {
    let mut options = svg2pdf::usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree =
        svg2pdf::usvg::Tree::from_str(&svg.to_string(), &options).expect("Error escribint el pdf");
    let pdf = svg2pdf::to_pdf(&tree, ConversionOptions::default(), PageOptions::default()).unwrap();
    std::fs::write(out_path, pdf).expect("hola");
    Ok(())
}
