# Manifold

## How to use (so far)
```Rust
use manifold::{matrix::Matrix, layer::Layer, activations::Activation, errors::ManifoldError, network::Network};

fn main() -> Result<(), ManifoldError> {
    // Input
    let open = 35.0;
    let close = 37.0;
    let high = 37.4;
    let low = 34.8;

    let input = Matrix::from([[
        open - close,   open - high, 
        open - low,     close - high, 
        close - low,    high - low]]);

    // Neural Network
    let nn = Network::new(vec![
        Layer::new(6, 16, Activation::ReLU),
        Layer::new(16, 16, Activation::ReLU),
        Layer::new(16, 16, Activation::ReLU),
        Layer::new(16, 16, Activation::ReLU),
        Layer::new(16, 3, Activation::SoftMax),
    ]);

    // One forward pass
    let res = nn.forward(input.clone())?;

    println!("Input: \n{input}\n");
    println!("Result: \n{res}");

    Ok(())
}
```

Output:
```
Input: 
[
-2.0000 -2.4000 0.2000 -0.4000 2.2000 2.6000
]

Result:
[
0.3060 0.3212 0.3728
]
```