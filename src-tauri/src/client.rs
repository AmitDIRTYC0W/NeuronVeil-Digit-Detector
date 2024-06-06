use anyhow::{anyhow, Context as _};
use log::debug;
use ndarray::{array, Array, Array1};
use neuronveil::message::Message;
use neuronveil::model::Model;
use neuronveil::utils::softmax;
use neuronveil::Com;
use ring::rand::{SecureRandom, SystemRandom};
use s2n_quic::{client::Connect, Client};
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::{error::Error, net::SocketAddr, path::Path, sync::Arc, time::Duration};
use tokio::sync::{mpsc, Mutex};
use tokio::task::LocalSet;
use tokio::task_local;
use tokio::time::sleep;

#[tauri::command]
pub async fn evaluate(pixels: Vec<f32>) -> Result<Vec<f32>, String> {
    sleep(Duration::from_millis(1000)).await;

    let output = tokio::task::spawn_blocking(move || -> anyhow::Result<Vec<f32>> {
        // Convert the input from float to Com
        let input = Array1::from_vec(pixels);
        let input_com = input.mapv(Com::from_num);

        debug!("input: {:#?}", input_com);

        // Read the model file
        let model_file =
            File::open("/home/user/Desktop/Cyber/Secure Inference/neuronveil/models/mnist.json")
                .context("Failed to read the model file")?;
        let reader = BufReader::new(model_file);
        let model: Model =
            serde_json::from_reader(reader).context("Failed to parse the model file")?;

        // Infer locally
        let output_com = model.infer_locally(input_com);

        // Convert the output from Com to float
        let output = output_com.mapv(Com::to_num::<f32>) / 2e6;

        // let minimum = output
        //     .iter()
        //     .min_by(|a, b| a.partial_cmp(b).unwrap())
        //     .unwrap();
        // debug!("Min: {}", minimum);
        // let shifted_output = &output - *minimum;
        // debug!("Shifted output: {:#?}", shifted_output);
        // let maximum = shifted_output
        //     .iter()
        //     .max_by(|a, b| a.partial_cmp(b).unwrap())
        //     .unwrap();
        // let normalised_output = &shifted_output / *maximum;

        let normalised_output = softmax(&output.view())
            .ok_or(anyhow!("Tried to softmax an empty array"))?
            .to_vec();

        debug!("Output: {:#} - {:?}", output, normalised_output);

        // Normalise the output
        Ok(normalised_output.to_vec())
    })
    .await
    .map_err::<String, _>(|_| "Failed to spawn a new thread".into())?
    .map_err::<String, _>(|e| format!("{:#}", e))?;

    Ok(output)
}

// async fn actually_evaluate(pixels: Vec<f32>) -> Result<Vec<f32>, ()> {
//     println!("FUCKKKKK");
//     debug!("Initialising the CSPRNG");
//     let system_random = Arc::new(SystemRandom::new());
//     let mut random_buffer = [0u8; 4];
//     system_random.fill(&mut random_buffer).unwrap();

//     let client = Client::builder()
//         .with_tls(Path::new("cert.pem"))
//         .unwrap()
//         .with_io("0.0.0.0:0")
//         .unwrap()
//         .start()
//         .unwrap();

//     debug!("Attempting to connect...");
//     let addr: SocketAddr = "127.0.0.1:1967".parse().unwrap();
//     let connect = Connect::new(addr).with_server_name("localhost");
//     let connection = client.connect(connect).await.unwrap();

//     let (mut connection_handle, mut stream_acceptor) = connection.split();

//     // Prepare for listening
//     let (incoming_sender, mut incoming_receiver) = mpsc::channel(1024); // TODO 1024 is a magic number

//     tokio::spawn(async move {
//         while let Ok(Some(mut stream)) = stream_acceptor.accept_receive_stream().await {
//             // Fully receive the message
//             let mut buffer: Vec<u8> = vec![];
//             tokio::io::copy(&mut stream, &mut buffer).await.unwrap();

//             // Parse it
//             let message: Message = serde_json::from_slice(&buffer).unwrap();
//             debug!("Received a message: {:?}", message);

//             // Process it
//             incoming_sender.send(message).await.unwrap();
//         }
//     });

//     // Prepare for sending
//     let (outcoming_sender, mut outcoming_receiver) = mpsc::channel(1024); // TODO 1024 is a magic number

//     tokio::spawn(async move {
//         while let Some(message) = outcoming_receiver.recv().await {
//             let mut stream = connection_handle.open_send_stream().await.unwrap(); // TODO handle errors!

//             let buffer = serde_json::to_vec(&message).unwrap().try_into().unwrap();
//             debug!("Attempting to send a message!");

//             stream.send(buffer).await.expect("stream should be open");
//             stream.close().await.unwrap();
//         }
//     });

//     let output = neuronveil::client::infer(
//         (&outcoming_sender, &mut incoming_receiver),
//         array![1f32, 1f32, -1f32, -1f32],
//         system_random.as_ref(),
//     )
//     .await
//     .unwrap();

//     Ok(output.into_raw_vec())
// }
