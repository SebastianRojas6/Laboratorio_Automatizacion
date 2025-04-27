use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashSet;
use crate::{random_utils::random_date, writer::create_writer};

pub fn gen_libro_autor_tesis(
    n: u32,
    libro_ids: &[u32],
    autor_tesis_ids: &[u32],
) -> Result<()> {
    let mut w = create_writer("libro_autor_tesis.csv")?;
    let mut rng = thread_rng();
    let mut used_combinations = HashSet::new();
    
    let mut count = 0;
    while count < n {
        let libro = *libro_ids.choose(&mut rng).unwrap();
        let autor_tesis = *autor_tesis_ids.choose(&mut rng).unwrap();
        
        let combination_key = format!("{}-{}", autor_tesis, libro);
        if used_combinations.contains(&combination_key) {
            continue;
        }
        used_combinations.insert(combination_key);
        
        let precio = format!("{:.2}", rng.gen_range(15.0..120.0));
        w.write_record(&[
            autor_tesis.to_string(),
            libro.to_string(),
            precio,
            random_date(),
        ])?;
        
        count += 1;
    }
    
    Ok(())
}
