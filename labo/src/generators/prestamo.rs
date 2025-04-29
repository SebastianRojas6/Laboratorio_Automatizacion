use anyhow::Result;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand::rngs::StdRng;
use rayon::prelude::*;
use crate::{random_utils::random_date, writer::create_writer};
use rand::thread_rng;

pub fn gen_prestamo(n: u32, libro_ids: &[u32], estudiante_codes: &[String]) -> Result<()> {
    let libro_ids = libro_ids.to_vec(); // rayon requiere propiedad
    let estudiante_codes = estudiante_codes.to_vec();

    let seed: [u8; 32] = thread_rng().gen();

    let prestamos: Vec<Vec<String>> = (0..n)
        .into_par_iter()
        .map_init(
            || StdRng::from_seed(seed),
            |rng, _| {
                let libro = *libro_ids.choose(rng).unwrap();
                let cod = estudiante_codes.choose(rng).unwrap();
                let codigo_prestamo = format!("{:0>6}", rng.gen_range(1..999999));
                let anio = rng.gen_range(2020..=2025);

                vec![
                    libro.to_string(),
                    cod.to_string(),
                    random_date(),
                    random_date(),
                    codigo_prestamo,
                    anio.to_string(),
                ]
            },
        )
        .collect();

    let mut w = create_writer("prestamo.csv")?;
    for row in prestamos {
        w.write_record(&row)?;
    }

    Ok(())
}
