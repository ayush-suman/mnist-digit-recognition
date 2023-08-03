mod image;
mod utils;
mod knn_classifier;
mod classifier;

use classifier::Classifier;
use knn_classifier::KNNClassifier;
use utils::load_test_images;

use crate::{image::Image, utils::load_training_images};

#[tokio::main]
async fn main() {
    let images: Vec<Image> = load_training_images();
    let test_images: Vec<Image> = load_test_images();

    let mut knn_classifier = KNNClassifier::new(1);

    for (i, image) in images.iter().enumerate() {
        knn_classifier.learn(image.data.to_vec(), image.label);
    }

    let mut correct: u32 = 0;

    for i in 0..10000 {
        let label = knn_classifier.predict(&test_images[i].data);
        if test_images[i].label == label {
            correct += 1;
            println!("Correct {} - {}", &test_images[i].label, &label);
        } else {
            println!("Incorrect Prediction");
        }
    }

    println!("Accuracy rate: {}", correct as f64 / 10000.0)

}
