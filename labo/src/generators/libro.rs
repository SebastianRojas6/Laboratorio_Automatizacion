use anyhow::Result;
use fakeit::words;
use rand::{seq::SliceRandom, thread_rng, Rng};
use crate::writer::create_writer;

pub fn gen_libro(first_id: u32, n: u32, editorial_ids: &[u32]) -> Result<()> {
    let mut rng = thread_rng();
    let mut w = create_writer("libro.csv")?;
    for id in first_id..first_id + n {
        let isbn = format!(
            "{}-{}",
            rng.gen_range(100000..999999),
            rng.gen_range(1000..9999)
        );
        let edi = editorial_ids.choose(&mut rng).unwrap();
        w.write_record(&[
            id.to_string(),
            words::sentence(20),
            isbn,
            edi.to_string(),
        ])?;
    }
    Ok(())
}
