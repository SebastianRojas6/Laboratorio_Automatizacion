use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use csv::WriterBuilder;
use fakeit::{address, company, contact, datetime, name, words};
use rand::{seq::SliceRandom, Rng};
use std::{fs::File, io::BufWriter, path::Path};
use std::collections::HashSet;

type CsvW = csv::Writer<BufWriter<File>>;

fn writer(path: &str) -> Result<CsvW> {
    std::fs::create_dir_all("output")?;
    let f = File::create(Path::new("output").join(path))?;
    Ok(WriterBuilder::new()
        .delimiter(b'|')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(BufWriter::new(f)))
}

fn random_date() -> String {
    let d = datetime::date();
    let ndt = NaiveDateTime::from_timestamp_opt(d.secs, d.nsecs).unwrap();
    let dt: DateTime<Utc> = DateTime::<Utc>::from_utc(ndt, Utc);
    dt.date_naive().to_string() // Así lo genera la librería -> "YYYY-MM-DD"
}

// Data random

fn gen_departamento(n: u32) -> Result<()> {
    let mut w = writer("departamento.csv")?;
    for id in 1..=n {
        w.write_record(&[id.to_string(), address::state()])?;
    }
    Ok(())
}

fn gen_editorial(first_id: u32, n: u32) -> Result<()> {
    let mut w = writer("editorial.csv")?;
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

fn gen_estudiante(first_num: u32, n: u32) -> Result<()> {
    let mut w = writer("estudiante.csv")?;
    for idx in 0..n {
        let codigo = format!("{:0>6}", first_num + idx);
        w.write_record(&[
            codigo.clone(),
            name::first(),
            name::last(),
            name::last(),
            address::street(),
            contact::phone(),
            ["A", "I"].choose(&mut rand::thread_rng()).unwrap().to_string(),
            rand::thread_rng().gen_range(10_000_000..99_999_999).to_string(),
        ])?;
    }
    Ok(())
}

fn gen_autor(first_id: u32, n: u32, dept_ids: &[u32]) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut w = writer("autor.csv")?;
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

fn gen_libro(first_id: u32, n: u32, editorial_ids: &[u32]) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut w = writer("libro.csv")?;
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

fn gen_prestamo(
    n: u32,
    libro_ids: &[u32],
    estudiante_codes: &[String],
) -> Result<()> {
    let mut w = writer("prestamo.csv")?;
    let mut rng = rand::thread_rng();
    let mut used_combinations = HashSet::new();
    
    let mut count = 0;
    while count < n {
        let libro = *libro_ids.choose(&mut rng).unwrap();
        let cod = estudiante_codes.choose(&mut rng).unwrap();
        let codigo_prestamo: String = format!("{:0>6}", rng.gen_range(1..999999));
        let anio = rng.gen_range(2020..=2025);
        
        // La clave primaria compuesta es (codigo_prestamo, anio_prestamo)
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

fn gen_autor_libro(
    first_id: u32,
    n: u32,
    libro_ids: &[u32],
    autor_ids: &[u32],
) -> Result<()> {
    let mut w = writer("autor_libro.csv")?;
    let mut rng = rand::thread_rng();
    let mut used_combinations = HashSet::new();
    
    let mut count = 0;
    let mut id = first_id;
    
    while count < n {
        let libro = *libro_ids.choose(&mut rng).unwrap();
        let autor = *autor_ids.choose(&mut rng).unwrap();
        
        // Verificar que no se repita la combinación autor-libro
        let combination_key = format!("{}-{}", autor, libro);
        
        if used_combinations.contains(&combination_key) {
            continue;
        }
        
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

fn gen_autor_tesis(first_id: u32, n: u32, dept_ids: &[u32]) -> Result<()> {
    let mut w = writer("autor_tesis.csv")?;
    let mut rng = rand::thread_rng();
    for id in first_id..first_id + n {
        let dep = dept_ids.choose(&mut rng).unwrap();
        w.write_record(&[
            id.to_string(),
            name::first(),
            name::last(),
            name::last(),
            random_date(),        
            random_date(),        
            dep.to_string(),
        ])?;
    }
    Ok(())
}

fn gen_libro_autor_tesis(
    n: u32,
    libro_ids: &[u32],
    autor_tesis_ids: &[u32],
) -> Result<()> {
    let mut w = writer("libro_autor_tesis.csv")?;
    let mut rng = rand::thread_rng();
    let mut used_combinations = HashSet::new();
    
    let mut count = 0;
    while count < n {
        let libro = *libro_ids.choose(&mut rng).unwrap();
        let autor_tesis = *autor_tesis_ids.choose(&mut rng).unwrap();
        
        // La clave primaria compuesta es (autor_tesis_id, libro_id)
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

fn main() -> Result<()> {
    // Parámetros (por si acaso xdd)
    let n_dept = 25;
    let n_editorial = 50;
    let n_estudiante = 2_000;
    let n_autor = 400;
    let n_libro = 1_000;
    let n_prestamo = 5_000;
    let n_autor_libro = 2_000;
    let n_autor_tesis = 300;
    let n_libro_autor_tesis = 600;

    gen_departamento(n_dept)?;

    gen_editorial(1, n_editorial)?;
    let editorial_ids: Vec<u32> = (1..=n_editorial).collect();

    gen_estudiante(10_000, n_estudiante)?;

    let estudiante_codes: Vec<String> = (0..n_estudiante)
        .map(|i| format!("{:0>6}", 10_000 + i))
        .collect();
    
    let dept_ids: Vec<u32> = (1..=n_dept).collect();
    gen_autor(1, n_autor, &dept_ids)?;
    let autor_ids: Vec<u32> = (1..=n_autor).collect();
    
    gen_libro(1, n_libro, &editorial_ids)?; 
    
    let libro_ids: Vec<u32> = (1..=n_libro).collect();

    gen_prestamo(n_prestamo, &libro_ids, &estudiante_codes)?;

    gen_autor_libro(1, n_autor_libro, &libro_ids, &autor_ids)?;

    gen_autor_tesis(1, n_autor_tesis, &dept_ids)?;
    let autor_tesis_ids: Vec<u32> = (1..=n_autor_tesis).collect();

    gen_libro_autor_tesis(n_libro_autor_tesis, &libro_ids, &autor_tesis_ids)?;

    println!("✅ CSV generados en ./output/");
    Ok(())
}