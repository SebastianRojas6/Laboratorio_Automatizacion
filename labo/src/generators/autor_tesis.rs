use anyhow::Result;
use fakeit::name;
use rand::{seq::SliceRandom, thread_rng};
use crate::{random_utils::random_date, writer::create_writer};

pub fn gen_autor_tesis(first_id: u32, n: u32, dept_ids: &[u32]) -> Result<()> {
    let mut w = create_writer("autor_tesis.csv")?;
    let mut rng = thread_rng();
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
