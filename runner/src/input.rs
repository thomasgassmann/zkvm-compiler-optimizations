use ndarray::Array2;
use sp1_sdk::SP1Stdin;

use crate::types::ProgramId;
use mnist::MnistBuilder;

fn load_mnist() -> (Vec<(Vec<f64>, Vec<f64>)>, Vec<(Vec<f64>, Vec<f64>)>) {
    let train_size: usize = 20;
    let test_size: usize = 4;
    let mnist = MnistBuilder::new()
        .training_set_length(train_size as u32)
        .test_set_length(test_size as u32)
        .base_url("https://systemds.apache.org/assets/datasets/mnist")
        .download_and_extract()
        .finalize();

    let mut train: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();
    let mut test: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();

    let train_data = Array2::from_shape_vec((train_size, 784), mnist.trn_img)
        .expect("Error converting images to Array2 struct")
        .map(|x| *x as f32 / 256.0);

    let train_labels: Array2<f32> = Array2::from_shape_vec((train_size, 1), mnist.trn_lbl)
        .expect("Error converting training labels to Array2 struct")
        .map(|x| *x as f32);

    let test_data = Array2::from_shape_vec((test_size, 784), mnist.tst_img)
        .expect("Error converting images to Array2 struct")
        .map(|x| *x as f32 / 256.0);

    let test_labels: Array2<f32> = Array2::from_shape_vec((test_size, 1), mnist.tst_lbl)
        .expect("Error converting training labels to Array2 struct")
        .map(|x| *x as f32);

    for i in 0..train_size {
        let image_data: Vec<f64> = train_data.row(i).iter().map(|&x| x as f64).collect();

        let mut label_data = vec![0.0; 10];
        let label = train_labels[(i, 0)] as usize;
        label_data[label] = 1.0;

        train.push((image_data, label_data));
    }

    for i in 0..test_size {
        let image_data: Vec<f64> = test_data.row(i).iter().map(|&x| x as f64).collect();

        let mut label_data = vec![0.0; 10];
        let label = test_labels[(i, 0)] as usize;
        label_data[label] = 1.0;

        test.push((image_data, label_data));
    }

    (train, test)
}

pub fn get_sp1_stdin(program: &ProgramId) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    match program {
        ProgramId::Factorial => {
            stdin.write::<u32>(&10);
        }
        ProgramId::Keccak256 => {
            stdin.write(&vec![0u8; 64]);
        }
        ProgramId::ZkvmMnist => {
            let (train, test) = load_mnist();
            stdin.write(&train);
            stdin.write(&test);
        },
        ProgramId::Bigmem => {
            stdin.write::<u32>(&42);
        },
        _ => {}
    }

    stdin
}

pub fn set_risc0_input(program: &ProgramId, builder: &mut risc0_zkvm::ExecutorEnvBuilder<'_>) {
    match program {
        ProgramId::Factorial => {
            let _ = builder.write::<u32>(&10);
        }
        ProgramId::Keccak256 => {
            let _ = builder.write(&vec![0u8; 64]);
        }
        ProgramId::ZkvmMnist => {
            let (train, test) = load_mnist();
            let _ = builder.write(&train);
            let _ = builder.write(&test);
        },
        ProgramId::Bigmem => {
            let _ = builder.write::<u32>(&42);
        },
        _ => {}
    }
}
