use std::fs::File;

use serde::Deserialize;

use crate::spectrum2d;

#[derive(Debug, Deserialize)]
struct IrRecord {
    wavenumber: f64,
    intensity: f64,
}

pub fn load_ir(file_path: String) -> spectrum2d::Spec2d {
    let spec_path = file_path;
    // todo graceful error handling
    let file = File::open(spec_path).unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(file);

    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    for result in rdr.deserialize() {
        let record: IrRecord = result.unwrap();
        x.push(record.wavenumber);
        y.push(record.intensity);
    }

    spectrum2d::Spec2d::new(
        "Testspec".to_string(),
        "wavenumber".to_string(),
        "intensity".to_string(),
        x,
        y,
    )
}
