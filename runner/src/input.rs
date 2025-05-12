use std::path::PathBuf;

use ndarray::{s, Array1, Array2};
use rsp_client_executor::io::ClientExecutorInput;
use serde::Serialize;
use sp1_sdk::SP1Stdin;

use k256::ecdsa::signature::SignerMut;

use crate::types::{ProgramId, ProverId};
use mnist::MnistBuilder;
use rand::{distributions::Alphanumeric, Rng};

fn downsample_image(image: &Array1<f32>) -> Vec<f64> {
    let image_2d = image
        .to_owned()
        .into_shape_with_order((28, 28))
        .expect("Error reshaping the image to 28x28");

    let mut downsampled = Vec::with_capacity(49);

    for i in 0..7 {
        for j in 0..7 {
            // Slice out a 4x4 block from the image
            let block = image_2d.slice(s![i * 4..i * 4 + 4, j * 4..j * 4 + 4]);
            // Sum the block elements, take the average (16 pixels per block)
            let sum: f32 = block.iter().sum();
            let avg = sum / 16.0;
            downsampled.push(avg as f64);
        }
    }

    downsampled
}

fn load_mnist() -> (Vec<(Vec<f64>, Vec<f64>)>, Vec<(Vec<f64>, Vec<f64>)>) {
    let train_size: usize = 150;
    let test_size: usize = 10;
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
        .expect("Error converting test labels to Array2 struct")
        .map(|x| *x as f32);

    for i in 0..train_size {
        let image_flat = train_data.row(i).to_owned();
        let image_data = downsample_image(&image_flat);

        let mut label_data = vec![0.0; 10];
        let label = train_labels[(i, 0)] as usize;
        label_data[label] = 1.0;

        train.push((image_data, label_data));
    }

    for i in 0..test_size {
        let image_flat = test_data.row(i).to_owned();
        let image_data = downsample_image(&image_flat);

        let mut label_data = vec![0.0; 10];
        let label = test_labels[(i, 0)] as usize;
        label_data[label] = 1.0;

        test.push((image_data, label_data));
    }

    (train, test)
}

pub fn load_rsp_input(file: &str) -> Vec<u8> {
    /*
       Cycle counts for rsp in o3 with respective inputs:
       input        - updated  - current
       22317400-bin - 3526886  - 4362939
       22287700-bin - 37250156 - 55353428
       20526624-bin - 41116545 - 59700433
       22264800-bin - 60895845 - 82308159
       22302600-bin - 54541791 - 80161036
       22343400-bin - 75719346 - 107782385
       22323700-bin - 64429101 - 94400495

       middle column represents cycle count we can get when
       updating rsp to latest version (sp1, o3)
       however, this breaks risc0, hence we still use the prev.
       version which is the current column
    */
    println!("Loading rsp input from file: {file}");
    let cache_path = PathBuf::from(format!("./inputs/rsp/{file}.bin"));
    let mut cache_file = std::fs::File::open(cache_path).unwrap();
    let client_input: ClientExecutorInput = bincode::deserialize_from(&mut cache_file).unwrap();
    bincode::serialize(&client_input).unwrap()
}

pub fn rand_ecdsa_signature() -> (k256::EncodedPoint, Vec<u8>, k256::ecdsa::Signature) {
    use k256::ecdsa::{SigningKey, VerifyingKey};
    use rand::rngs::OsRng;

    let mut signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);

    let message = b"Hello, world!";
    let signature = signing_key.sign(message);

    (
        verifying_key.to_encoded_point(true),
        message.to_vec(),
        signature,
    )
}

pub fn rand_eddsa_signature() -> (
    ed25519_dalek::VerifyingKey,
    Vec<u8>,
    ed25519_dalek::Signature,
) {
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    let signing_key = SigningKey::generate(&mut OsRng);
    let message = b"Hello, world!";
    let signature = signing_key.sign(message);

    (signing_key.verifying_key(), message.to_vec(), signature)
}

pub trait ProgramInputWriter {
    fn write_string(&mut self, s: &str);
    fn write_generic<T: Serialize>(&mut self, value: &T);
    fn write_vec(&mut self, input: Vec<u8>);
}

impl ProgramInputWriter for SP1Stdin {
    fn write_string(&mut self, s: &str) {
        self.write(&String::from(s));
    }

    fn write_generic<T: Serialize>(&mut self, value: &T) {
        self.write(value);
    }

    fn write_vec(&mut self, input: Vec<u8>) {
        self.write_vec(input);
    }
}

impl<'a> ProgramInputWriter for risc0_zkvm::ExecutorEnvBuilder<'a> {
    fn write_string(&mut self, s: &str) {
        let _ = self.write(&String::from(s));
    }

    fn write_generic<T: Serialize>(&mut self, value: &T) {
        let _ = self.write(value);
    }

    fn write_vec(&mut self, input: Vec<u8>) {
        let _ = self.write(&input);
    }
}

pub fn get_sp1_stdin(program: &ProgramId, input_override: &Option<String>) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    write_program_inputs(program, &mut stdin, ProverId::SP1, input_override);
    stdin
}

pub fn set_risc0_input(
    program: &ProgramId,
    builder: &mut risc0_zkvm::ExecutorEnvBuilder<'_>,
    input_override: &Option<String>,
) {
    write_program_inputs(program, builder, ProverId::Risc0, input_override);
}

fn write_program_inputs<W: ProgramInputWriter>(
    program: &ProgramId,
    stdin: &mut W,
    _: ProverId,
    input_override: &Option<String>,
) {
    match program {
        ProgramId::LoopSum => {
            let mut arr = Vec::new();
            for i in 0..1500 {
                arr.push(i);
            }

            stdin.write_generic(&arr);
        }
        ProgramId::Factorial => {
            stdin.write_generic(&1000u32);
        }
        ProgramId::Tailcall => {
            stdin.write_generic(&25u128);
            stdin.write_generic(&300u128);
        }
        ProgramId::Keccak256 => {
            stdin.write_generic(&vec![0u8; 64]);
        }
        ProgramId::ZkvmMnist => {
            let (train, test) = load_mnist();
            stdin.write_generic(&train);
            stdin.write_generic(&test);
        }
        ProgramId::Bigmem => {
            stdin.write_generic(&42u32);
        }
        ProgramId::Fibonacci => {
            stdin.write_generic(&30000u32);
        }
        ProgramId::Sha2Bench => {
            stdin.write_generic(&vec![5u8; 64]);
        }
        ProgramId::Sha3Bench => {
            stdin.write_generic(&vec![5u8; 64]);
        }
        ProgramId::Sha2Chain => {
            stdin.write_generic(&vec![5u8; 32]);
            stdin.write_generic(&32u32);
        }
        ProgramId::Sha3Chain => {
            stdin.write_generic(&vec![5u8; 32]);
            stdin.write_generic(&32u32);
        }
        ProgramId::RegexMatch => {
            // sample from https://docs.rs/regex/latest/regex/
            stdin.write_string("[0-9]{4}-[0-9]{2}-[0-9]{2}");
            stdin.write_string(
                "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?",
            );
        }
        ProgramId::Rsp => {
            if input_override.is_some() {
                stdin.write_vec(load_rsp_input(input_override.as_ref().unwrap()));
            } else {
                stdin.write_vec(load_rsp_input("20526624"));
            }
        }
        ProgramId::Merkle => {
            let mut rng = rand::thread_rng();
            const MAX_STRINGS: u32 = 25;
            let strings: Vec<String> = (0..MAX_STRINGS)
                .map(|_| {
                    (0..10) // Generate strings of length 10
                        .map(|_| rng.sample(Alphanumeric) as char)
                        .collect()
                })
                .collect();

            stdin.write_generic(&strings);
            let range: std::ops::Range<usize> = 10..13 as usize;
            stdin.write_generic(&range);
        }
        ProgramId::EcdsaVerify => {
            stdin.write_generic(&rand_ecdsa_signature());
        }
        ProgramId::EddsaVerify => {
            let times: u8 = 10;
            stdin.write_generic(&times);

            for _ in 0..times {
                stdin.write_generic(&rand_eddsa_signature());
            }
        }
        ProgramId::Spec619 => {
            stdin.write_generic(&1); // timesteps
            stdin.write_generic(&0); // action: NOTHING = 0, COMPARE, STORE
            stdin.write_generic(&0); // simType: LDC = 0, CHANNEL
        }
        ProgramId::Spec631 => {
            let str = include_str!("../../inputs/spec-631/in.txt");
            stdin.write_string(str);
        }
        _ => {}
    }
}
