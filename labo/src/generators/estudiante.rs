use anyhow::Result;
use fakeit::{address, contact, name};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand::rngs::StdRng;
use rayon::prelude::*;
use crate::writer::create_writer;

pub fn gen_estudiante(first_num: u32, n: u32, rng: &mut impl Rng) -> Result<()> {
    let seed: [u8; 32] = rng.gen();

    let estudiantes: Vec<Vec<String>> = (0..n)
        .into_par_iter()
        .map_init(
            || StdRng::from_seed(seed),
            |rng_local, idx| {
                let codigo = format!("{:0>6}", first_num + idx);
                vec![
                    codigo,
                    name::first(),
                    name::last(),
                    name::last(),
                    address::street(),
                    contact::phone(),
                    ["A", "I"].choose(rng_local).unwrap().to_string(),
                    rng_local.gen_range(10_000_000..99_999_999).to_string(),
                ]
            },
        )
        .collect();

    let mut w = create_writer("estudiante.csv")?;
    for est in estudiantes {
        w.write_record(&est)?;
    }

    Ok(())
}
