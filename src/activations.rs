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

fn linear(_: &mut [f32]) {
    
}

fn sigmoid(x: &mut [f32]) {
    for i in 0..x.len() {
        x[i] = 1.0 / (1.0 + E.powf(-x[i]));
    }
}

fn tanh(x: &mut [f32]) {
    for i in 0..x.len() {
        x[i] = x[i].tanh();
    }
}

fn relu(x: &mut [f32]) {
    for i in 0..x.len() {
        if x[i] < 0.0 {
            x[i] = 0.0;
        } else {
            x[i] = x[i];
        }
    }
}

fn softmax(x: &mut [f32]) {
    let mut sum = 0.0;
    for i in x.iter() {
        sum += *i;
    }

    for i in 0..x.len() {
        x[i] /= sum;
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