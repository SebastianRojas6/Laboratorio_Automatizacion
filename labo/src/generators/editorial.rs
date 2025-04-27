use anyhow::Result;
use fakeit::{address, company, contact};
use crate::writer::create_writer;

pub fn gen_editorial(first_id: u32, n: u32) -> Result<()> {
    let mut w = create_writer("editorial.csv")?;
    for id in first_id..first_id + n {
        w.write_record(&[
            id.to_string(),
            company::company(),
            address::street(),
            contact::phone(),
        ])?;
    }
    Ok(())
}
