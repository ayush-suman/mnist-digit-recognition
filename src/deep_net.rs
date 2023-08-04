use crate::classifier::Classifier;

pub struct ANN {
    pub node_counts: Vec<usize>,
    node_weights: Vec<Vec<Vec<u8>>>,
    learning_rate: f64
}

enum Weights {
    Default,
    As(Vec<Vec<Vec<u8>>>)
}

impl ANN {
    fn new(node_counts: Vec<usize>, learning_rate: f64, weights: Weights) -> ANN {
        match weights {
            Weights::Default => {
                let mut node_weights = Vec::with_capacity(node_counts.len() - 1);
                for i in 0..node_counts.len() - 1 {
                    node_weights.push(Vec::with_capacity(node_counts[i]));
                    for j in 0..node_counts[i] {
                        node_weights[i].push(Vec::with_capacity(node_counts[i+1]));
                        for k in 0..node_counts[i+1] {
                            node_weights[i][j].push(0);
                        }
                    }
                }
                ANN { node_counts, node_weights, learning_rate }
            },
            Weights::As(node_weights) => {
                ANN { node_counts, node_weights, learning_rate }
            }
        } 
    }
}



impl Classifier<Vec<Vec<u8>>, u8> for ANN {
    fn learn(&mut self, data: Vec<Vec<u8>>, label: u8) {
        // TODO: Flatten data to 1-D,
        // TODO: Calculate forward up until the last layer
        // TODO: use ReLU activation fn
        // TODO: Calculate loss
        // TODO: use Hinge Loss
        // TODO: Calculate backward through backprop and update all weights 
    }

    fn predict(&self, data: &Vec<Vec<u8>>) -> u8 {
        // TODO: Flatten data to 1-D,
        // TODO: Calculate forward up until the last layer
        // TODO: use ReLU activation fn
        // TODO: return index of output with max value
        0
    }
}