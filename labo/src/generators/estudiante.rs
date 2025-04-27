use anyhow::Result;
use fakeit::{address, contact, name};
use rand::{seq::SliceRandom, Rng};
use crate::writer::create_writer;

pub fn gen_estudiante(first_num: u32, n: u32, rng: &mut impl Rng) -> Result<()> {
    let mut w = create_writer("estudiante.csv")?;
    for idx in 0..n {
        let codigo = format!("{:0>6}", first_num + idx);
        w.write_record(&[
            codigo.clone(),
            name::first(),
            name::last(),
            name::last(),
            address::street(),
            contact::phone(),
            ["A", "I"].choose(rng).unwrap().to_string(),
            rng.gen_range(10_000_000..99_999_999).to_string(),
        ])?;
    }
    Ok(())
}
