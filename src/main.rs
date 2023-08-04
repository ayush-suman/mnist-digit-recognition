mod image;
mod utils;
mod knn_classifier;
mod classifier;
mod ann;

use std::sync::Arc;

use tokio::sync::mpsc;
use classifier::Classifier;
use knn_classifier::KNNClassifier;
use utils::load_test_images;

use crate::{image::Image, utils::load_training_images, ann::{ANN, Weights}};

#[tokio::main]
async fn main() {
    let mut deep_net = ANN::new(vec![10, 10], 0.1, Weights::Default);

    deep_net.learn(vec![vec![0,1], vec![2,3]], 4);
    // let images: Vec<Image> = load_training_images();
    // let test_images: Vec<Image> = load_test_images();

    // let mut knn_classifier = KNNClassifier::new(1);

    // for image in images {
    //     knn_classifier.learn(image.data.to_vec(), image.label);
    // }

    // let knn_classifier_arc = Arc::new(knn_classifier);
    // let test_images_arc = Arc::new(test_images);

    // let (tx, mut rx) = mpsc::channel::<u32>(128);

    // let test_images_count = test_images_arc.len();

    // const PARALLEL_TASKS: usize = 10; // Should be less than no of CPUs [and divisible by test_images_count]

    // for i in 0..PARALLEL_TASKS {
    //     for j in (i * test_images_count / PARALLEL_TASKS)..(i * test_images_count / PARALLEL_TASKS + test_images_count / PARALLEL_TASKS) {
    //         let txc = tx.clone();
    //         let knn_classifier_arc = Arc::clone(&knn_classifier_arc);
    //         let test_images_arc = Arc::clone(&test_images_arc);
    //         tokio::spawn(async move {
    //             let label = knn_classifier_arc.predict(&test_images_arc[j].data);
    //             if test_images_arc[j].label == label {
    //                 txc.send(1).await.unwrap();
    //                 println!("Correct.");
    //             } else {
    //                 txc.send(0).await.unwrap();
    //                 println!("Incorrect. Predicted {} as {}", &label, &test_images_arc[j].label);
    //             }
    //         });   
    //     }
    // }

    // let mut i = 0;
    // let mut correct: u32 = 0;
    // while let Some(c) = rx.recv().await {
    //     correct += c;
    //     i += 1;
    //     if i == test_images_count {
    //         break;
    //     }
    // }

    // println!("Accuracy rate: {}", correct as f64 / 10000.0)

}
