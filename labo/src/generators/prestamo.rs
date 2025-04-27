use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashSet;
use crate::{random_utils::random_date, writer::create_writer};

pub fn gen_prestamo(n: u32, libro_ids: &[u32], estudiante_codes: &[String]) -> Result<()> {
    let mut w = create_writer("prestamo.csv")?;
    let mut rng = thread_rng();
    let mut used_combinations = HashSet::new();
    
    let mut count = 0;
    while count < n {
        let libro = *libro_ids.choose(&mut rng).unwrap();
        let cod = estudiante_codes.choose(&mut rng).unwrap();
        let codigo_prestamo = format!("{:0>6}", rng.gen_range(1..999999));
        let anio = rng.gen_range(2020..=2025);
        
        let combination_key = format!("{}-{}", codigo_prestamo, anio);
        
        if used_combinations.contains(&combination_key) {
            continue;
        }
        used_combinations.insert(combination_key);
        
        w.write_record(&[
            libro.to_string(),
            cod.to_string(),
            random_date(),
            random_date(),
            codigo_prestamo,
            anio.to_string(),
        ])?;
        
        count += 1;
    }
    
    Ok(())
}
