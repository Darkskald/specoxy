use pyo3::prelude::*;

#[pyclass]
pub struct Spec2d {
    // metadata
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub spec_type: String,
    #[pyo3(get)]
    pub x_label: String,
    #[pyo3(get)]
    pub y_label: String,

    // data
    #[pyo3(get)]
    pub x: Vec<f64>,
    #[pyo3(get)]
    pub y: Vec<f64>,
}

#[pymethods]
impl Spec2d {
    #[new]
    pub fn new(name: String, x_label: String, y_label: String, x: Vec<f64>, y: Vec<f64>) -> Self {
        Spec2d {
            name,
            spec_type: String::from("generic"),
            x_label,
            y_label,
            x,
            y,
        }
    }

    pub fn info(&self) -> String {
        format!(
            "spectrum {} with x={}, y={}",
            self.name, self.x_label, self.y_label
        )
    }

    pub fn do_sth(&self)-> Vec<f64> {
        self.x.iter()
        .map(|i| i*i)
        .collect()
    }
}
