use crate::{layer::Layer, matrix::Matrix, errors::ManifoldError};

pub struct Network {
    layers: Vec<Layer>
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers
        }
    }

    pub fn forward(&self, input: Matrix) -> Result<Matrix, ManifoldError> {
        let mut res = input;
        for layer in self.layers.iter() {
            res = layer.forward(res)?;
        }
        Ok(res)
    }
}