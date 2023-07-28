use std::f32::consts::E;

#[derive(Clone, Copy)]
pub enum Activation {
    Linear,
    Sigmoid,
    Tanh,
    ReLU,
    SoftMax
}

impl Activation {
    pub(crate) fn apply(&self, x: &mut [f32]) {
        match self {
            Activation::Linear => linear(x),
            Activation::Sigmoid => sigmoid(x),
            Activation::Tanh => tanh(x),
            Activation::ReLU => relu(x),
            Activation::SoftMax => softmax(x),
        }
    }
}

/// Updates the slice in place with the activation fn output
fn linear(_: &mut [f32]) {
    
}

/// Updates the slice in place with the derivation
fn linear_derivation(x: &mut [f32]) {
    x.iter_mut().for_each(|n| *n = 1.0);
}

fn sigmoid(x: &mut [f32]) {
    x.iter_mut().for_each(|f| *f = 1.0 / (1.0 + E.powf(-*f)));
}

fn sigmoid_derivation(x: &mut [f32]) {
    x.iter_mut().for_each(|f| *f = E.powf(-*f) / (1.0 + E.powf(-*f)).powi(2));
}

fn tanh(x: &mut [f32]) {
    x.iter_mut().for_each(|f| *f = (*f).tanh());
}

fn tanh_derivation(x: &mut [f32]) {
    x.iter_mut().for_each(|f| *f = 1.0 - (*f).tanh().powi(2));
}

fn relu(x: &mut [f32]) {
    x.iter_mut().for_each(|f| *f = if *f > 0.0 { *f } else { 0.0 });
}

fn relu_derivation(x: &mut [f32]) {
    x.iter_mut().for_each(|f| *f = if *f > 0.0 { 1.0 } else { 0.0 });
}

// TODO: Why does this give NAN?? 
// Cuz of big values...
// Need to use max normalization...
fn softmax(x: &mut [f32]) {
    let mut sum = 0.0;
    for i in x.iter() {
        sum += E.powf(*i);
    }

    for i in 0..x.len() {
        x[i] = E.powf(x[i]) / sum;
    }
}

#[cfg(test)]
mod tests {
    use super::linear;

    #[test]
    fn test_linear() {
        let mut x = vec![1.0, 2.0, 3.0];
        let control = x.clone();
        linear(&mut x);
        assert_eq!(x, control);
    }
}