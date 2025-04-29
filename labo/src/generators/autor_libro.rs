use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashSet;
use crate::{random_utils::random_date, writer::create_writer};

pub fn gen_autor_libro(
    first_id: u32,
    n: u32,
    libro_ids: &[u32],
    autor_ids: &[u32],) -> Result<()> {

    let mut w = create_writer("autor_libro.csv")?;
    let mut rng = thread_rng();
    let mut used_combinations = HashSet::new();
    
    let mut count = 0;
    let mut id = first_id;
    
    while count < n {
        let libro = *libro_ids.choose(&mut rng).unwrap();
        let autor = *autor_ids.choose(&mut rng).unwrap();
        
        let combination_key = format!("{}-{}", autor, libro);
        if used_combinations.contains(&combination_key) {
            continue;}
            
        used_combinations.insert(combination_key);
        
        let ejemplares = rng.gen_range(1..1000).to_string();
        let precio = format!("{:.2}", rng.gen_range(10.0..100.0));
        
        w.write_record(&[
            id.to_string(),
            libro.to_string(),
            autor.to_string(),
            ejemplares,
            random_date(),
            precio,
        ])?;
        
        id += 1;
        count += 1;
    }
    
    Ok(())
}
