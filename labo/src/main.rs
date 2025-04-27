mod writer;
mod random_utils;
mod generators;

use anyhow::Result;
use generators::{
    autor::gen_autor,
    autor_libro::gen_autor_libro,
    autor_tesis::gen_autor_tesis,
    departamento::gen_departamento,
    editorial::gen_editorial,
    estudiante::gen_estudiante,
    libro::gen_libro,
    libro_autor_tesis::gen_libro_autor_tesis,
    prestamo::gen_prestamo,
};

use rand::thread_rng;
use std::time::Instant;
use std::sync::Arc;
use std::thread;

fn main() -> Result<()> {
    let start = Instant::now();

    let n_dept = 50;
    let n_editorial = 5_000;
    let n_estudiante = 200_000;
    let n_autor = 150_000;
    let n_libro = 250_000;
    let n_prestamo = 500_000;
    let n_autor_libro = 200_000;
    let n_autor_tesis = 25_000;
    let n_libro_autor_tesis = 30_000;

    gen_departamento(n_dept)?;
    let dept_ids: Arc<Vec<u32>> = Arc::new((1..=n_dept).collect());

    let mut handles = vec![];

    // Editorial
    let editorial_handle = thread::spawn(move || -> Result<Vec<u32>> {
        gen_editorial(1, n_editorial)?;
        let editorial_ids: Vec<u32> = (1..=n_editorial).collect();
        Ok(editorial_ids)
    });

    // Estudiante
    let estudiante_handle = thread::spawn(move || -> Result<Vec<String>> {
        let mut rng = thread_rng();
        gen_estudiante(10_000, n_estudiante, &mut rng)?;
        let estudiante_codes: Vec<String> = (0..n_estudiante)
            .map(|i| format!("{:0>6}", 10_000 + i))
            .collect();
        Ok(estudiante_codes)
    });

    // Autor
    let dept_ids_clone = Arc::clone(&dept_ids);
    let autor_handle = thread::spawn(move || -> Result<Vec<u32>> {
        gen_autor(1, n_autor, &dept_ids_clone)?;
        let autor_ids: Vec<u32> = (1..=n_autor).collect();
        Ok(autor_ids)
    });

    // Esperamos a que terminen y obtenemos los resultados
    let editorial_ids = editorial_handle.join().unwrap()?;
    let estudiante_codes = estudiante_handle.join().unwrap()?;
    let autor_ids = autor_handle.join().unwrap()?;

    // Libro (depende de editorial)
    let editorial_ids = Arc::new(editorial_ids);
    let libro_handle = thread::spawn(move || -> Result<Vec<u32>> {
        gen_libro(1, n_libro, &editorial_ids)?;
        let libro_ids: Vec<u32> = (1..=n_libro).collect();
        Ok(libro_ids)
    });
    
    let libro_ids = Arc::new(libro_handle.join().unwrap()?);
    
    // Autor tesis (depende de departamentos)
    let dept_ids_clone = Arc::clone(&dept_ids);
    let autor_tesis_handle = thread::spawn(move || -> Result<Vec<u32>> {
        gen_autor_tesis(1, n_autor_tesis, &dept_ids_clone)?;
        let autor_tesis_ids: Vec<u32> = (1..=n_autor_tesis).collect();
        Ok(autor_tesis_ids)
    });
    
    let autor_tesis_ids = Arc::new(autor_tesis_handle.join().unwrap()?);
    
    // Las siguientes operaciones pueden ejecutarse en paralelo
    handles.clear();
    
    // Préstamo
    let libro_ids_clone = Arc::clone(&libro_ids);
    let estudiante_codes = Arc::new(estudiante_codes);
    let estudiante_codes_clone = Arc::clone(&estudiante_codes);
    handles.push(thread::spawn(move || -> Result<()> {
        gen_prestamo(n_prestamo, &libro_ids_clone, &estudiante_codes_clone)?;
        Ok(())
    }));
    
    // Autor libro
    let libro_ids_clone = Arc::clone(&libro_ids);
    let autor_ids = Arc::new(autor_ids);
    let autor_ids_clone = Arc::clone(&autor_ids);
    handles.push(thread::spawn(move || -> Result<()> {
        gen_autor_libro(1, n_autor_libro, &libro_ids_clone, &autor_ids_clone)?;
        Ok(())
    }));
    
    // Libro autor tesis
    let libro_ids_clone = Arc::clone(&libro_ids);
    let autor_tesis_ids_clone = Arc::clone(&autor_tesis_ids);
    handles.push(thread::spawn(move || -> Result<()> {
        gen_libro_autor_tesis(n_libro_autor_tesis, &libro_ids_clone, &autor_tesis_ids_clone)?;
        Ok(())
    }));
    
    // Esperamos a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap()?;
    }

    let duration = start.elapsed();
    println!("CSV generados en output/");
    println!("El tiempo total que se demoró fue: {:.2?}", duration);

    Ok(())
}