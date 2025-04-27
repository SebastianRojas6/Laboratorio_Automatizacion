use anyhow::Result;
use fakeit::address;
use crate::writer::create_writer;

pub fn gen_departamento(n: u32) -> Result<()> {
    let mut w = create_writer("departamento.csv")?;
    for id in 1..=n {
        w.write_record(&[id.to_string(), address::state()])?;
    }
    Ok(())
}
