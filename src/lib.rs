use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ValueType {
    Number(f64),
    Boolean(bool),
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyAgentState {
    vals: HashMap<String, ValueType>,
}

struct CompatibleTypes {
    num: PyResult<f64>,
    boolean: PyResult<bool>,
}
impl CompatibleTypes {
    fn from_val(val: &PyAny) -> Self {
        Self {
            num: val.extract(),
            boolean: val.extract(),
        }
    }
}

#[pymethods]
impl PyAgentState {
    #[new]
    pub fn new() -> Self {
        let vals = HashMap::new();
        Self { vals }
    }

    pub fn set(&mut self, field: &str, val: &PyAny) {
        match CompatibleTypes::from_val(val) {
            CompatibleTypes { num: Ok(v), .. } => {
                self.vals.insert(field.to_string(), ValueType::Number(v))
            }
            CompatibleTypes { boolean: Ok(v), .. } => {
                self.vals.insert(field.to_string(), ValueType::Boolean(v))
            }
            _ => unimplemented!(),
        };
    }

    pub fn get(&self, field: &str) {
        println!("field is {:#?}", self.vals.get(&(field.to_string())));
    }
}

#[pymodule]
fn hashcore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAgentState>()?;
    Ok(())
}
