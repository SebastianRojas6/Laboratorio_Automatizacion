use anyhow::Result;
use fakeit::{address, contact, name};
use rand::{seq::SliceRandom, thread_rng, Rng};
use crate::{random_utils::random_date, writer::create_writer};

pub fn gen_autor(first_id: u32, n: u32, dept_ids: &[u32]) -> Result<()> {
    let mut rng = thread_rng();
    let mut w = create_writer("autor.csv")?;
    for id in first_id..first_id + n {
        let dep = dept_ids.choose(&mut rng).unwrap();
        let estatura = format!("{:.2}", rng.gen_range(1.50..2.05));
        w.write_record(&[
            id.to_string(),
            name::first(),
            name::last(),
            random_date(),
            address::street(),
            contact::phone(),
            ["A", "I"].choose(&mut rng).unwrap().to_string(),
            estatura,
            dep.to_string(),
        ])?;
    }
    Ok(())
}
