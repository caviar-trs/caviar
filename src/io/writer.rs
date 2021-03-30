use crate::structs::ResultStructure;
use csv::Writer;
use std::error::Error;

#[allow(dead_code)]
pub fn write_results(path: &str, results: &Vec<ResultStructure>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    for result in results {
        wtr.serialize(result)?;
    }

    wtr.flush()?;

    Ok(())
}
