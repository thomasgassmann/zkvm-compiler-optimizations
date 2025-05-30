pub struct RNG {
    state: u64,
}

impl RNG {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        // simple LCG (from Numerical Recipes)
        // https://en.wikipedia.org/wiki/Linear_congruential_generator
        const A: u64 = 1664525;
        const C: u64 = 1013904223;

        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }

    pub fn next_f64(&mut self, min: f64, max: f64) -> f64 {
        let rand_u64 = self.next();
        let normalized = (rand_u64 as f64) / (u64::MAX as f64);
        min + (max - min) * normalized
    }
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

// computes sigmoid(a * x + b)
fn linear_and_sigmoid(a: &Vec<Vec<f64>>, x: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0; a.len()];
    for i in 0..a.len() {
        let mut sum = 0.0;
        for j in 0..x.len() {
            sum += a[i][j] * x[j];
        }

        result[i] = sigmoid(sum + b[i]);
    }
    result
}

pub struct NeuralNetwork {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
    weights_input_hidden: Vec<Vec<f64>>,
    weights_hidden_output: Vec<Vec<f64>>,
    biases_hidden: Vec<f64>,
    biases_output: Vec<f64>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        // matrices are stored as row-major
        let mut rng = RNG::new(42);
        let weights_input_hidden: Vec<Vec<f64>> = (0..hidden_size)
            .map(|_| (0..input_size).map(|_| rng.next_f64(-0.5, 0.5)).collect())
            .collect();
        let weights_hidden_output: Vec<Vec<f64>> = (0..output_size)
            .map(|_| (0..hidden_size).map(|_| rng.next_f64(-0.5, 0.5)).collect())
            .collect();
        let biases_hidden = vec![0.5; hidden_size];
        let biases_output = vec![0.5; output_size];

        Self {
            input_size,
            hidden_size,
            output_size,
            weights_input_hidden,
            weights_hidden_output,
            biases_hidden,
            biases_output,
        }
    }

    pub fn forward(&self, input: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        let hidden = linear_and_sigmoid(&self.weights_input_hidden, input, &self.biases_hidden);
        let output = linear_and_sigmoid(&self.weights_hidden_output, &hidden, &self.biases_output);
        (hidden, output)
    }

    pub fn test(&self, data: &Vec<(Vec<f64>, Vec<f64>)>) {
        let mut correct = 0;
        for (input, target) in data.iter() {
            let (_, output) = self.forward(input);

            let max_index = output
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;
            let target_index = target
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;
            if max_index == target_index {
                correct += 1;
            }
        }

        println!(
            "Accuracy: {}, correct: {}/{}",
            correct as f64 / data.len() as f64,
            correct,
            data.len()
        );
    }

    pub fn train(
        &mut self,
        data: &Vec<(Vec<f64>, Vec<f64>)>,
        test: &Vec<(Vec<f64>, Vec<f64>)>,
        epochs: usize,
        lr: f64,
    ) {
        for i in 0..epochs {
            println!("Epoch {}", i);
            for (input, target) in data.iter() {
                let (hidden, output) = self.forward(input);

                let output_errors: Vec<f64> = output
                    .iter()
                    .zip(target.iter())
                    .map(|(&o, &t)| (t - o) * sigmoid_derivative(o))
                    .collect();

                let hidden_errors: Vec<f64> = (0..self.hidden_size)
                    .map(|j| {
                        self.weights_hidden_output
                            .iter()
                            .enumerate()
                            .map(|(k, row)| row[j] * output_errors[k])
                            .sum::<f64>()
                            * sigmoid_derivative(hidden[j])
                    })
                    .collect();

                for k in 0..self.output_size {
                    for j in 0..self.hidden_size {
                        self.weights_hidden_output[k][j] += lr * hidden[j] * output_errors[k];
                    }
                }

                for j in 0..self.hidden_size {
                    for i in 0..self.input_size {
                        self.weights_input_hidden[j][i] += lr * input[i] * hidden_errors[j];
                    }
                }

                for k in 0..self.output_size {
                    self.biases_output[k] += lr * output_errors[k];
                }

                for j in 0..self.hidden_size {
                    self.biases_hidden[j] += lr * hidden_errors[j];
                }
            }

            println!("Finished epoch {}", i);
            self.test(&test);
        }
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
#[cfg(feature = "x86")]
pub extern "C" fn main_core(
    training_data: Vec<(Vec<f64>, Vec<f64>)>,
    test_data: Vec<(Vec<f64>, Vec<f64>)>,
) -> () {
    let mut nn = NeuralNetwork::new(49, 8, 10);
    nn.train(&training_data, &test_data, 2, 1.0);
}
