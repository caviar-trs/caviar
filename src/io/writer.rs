use csv::Writer;
use std::error::Error;
use crate::structs::ResultStructure;

pub fn write_results(path: &str, results: &Vec<ResultStructure>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    for result in results {
        wtr.serialize(result)?;
    }

    wtr.flush()?;

    Ok(())
}