use anyhow::Result;
use csv::WriterBuilder;
use std::{fs::File, io::BufWriter, path::Path};

pub type CsvW = csv::Writer<BufWriter<File>>;

pub fn create_writer(path: &str) -> Result<CsvW> {
    std::fs::create_dir_all("output")?;
    let f = File::create(Path::new("output").join(path))?;
    Ok(WriterBuilder::new()
        .delimiter(b'|')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(BufWriter::new(f)))
}

