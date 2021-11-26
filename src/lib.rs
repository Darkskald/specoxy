pub mod spectrum2d;
pub mod csv;

use pyo3::prelude::*;

#[pyfunction]
fn deliver_ir() -> PyResult<spectrum2d::Spec2d>{
    Ok(csv::load_ir())
}

#[pymodule]
fn specoxy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<spectrum2d::Spec2d>()?;
    m.add_function(wrap_pyfunction!(deliver_ir, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
