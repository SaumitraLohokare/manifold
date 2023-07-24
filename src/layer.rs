use crate::{matrix::Matrix, activations::Activation, errors::ManifoldError};

/// This structure encapsulates one layer of a neural network
#[derive(Clone)]
pub struct Layer {
    weights: Matrix,
    biases:  Matrix,
    activation: Activation
}

impl Layer {
    /// Creates a new `Layer` with random weights and biases
    pub fn new(i: usize, o: usize, activation: Activation) -> Self {
        Self {
            weights: Matrix::new_rand(i, o),
            biases:  Matrix::new_rand(1, o),
            activation
        }
    }

    pub fn forward(&self, input: Matrix) -> Result<Matrix, ManifoldError> {
        let mut m = input.mul(&self.weights)?.add(&self.biases)?;
        m.apply_activation(self.activation);
        Ok(m)
    }
}