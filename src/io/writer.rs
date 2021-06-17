use crate::structs::{PaperResult, ResultStructure};
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

#[allow(dead_code)]
pub fn write_results_paper(path: &str, results: &Vec<PaperResult>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    for result in results {
        wtr.serialize(result)?;
    }

    wtr.flush()?;

    Ok(())
}
