use std::{f64::consts::E, intrinsics::log10f64};

use crate::classifier::Classifier;

pub struct ANN {
    pub node_counts: Vec<usize>,
    node_weights: Vec<Vec<Vec<f64>>>,
    learning_rate: f64
}

pub enum Weights {
    Default,
    As(Vec<Vec<Vec<f64>>>)
}

impl ANN {
    pub fn new(node_counts: Vec<usize>, learning_rate: f64, weights: Weights) -> ANN {
        match weights {
            Weights::Default => {
                let mut node_weights: Vec<Vec<Vec<f64>>> = Vec::with_capacity(node_counts.len() - 1);
                for i in 0..node_counts.len() - 1 {
                    node_weights.push(Vec::with_capacity(node_counts[i]));
                    for j in 0..node_counts[i] {
                        node_weights[i].push(Vec::with_capacity(node_counts[i+1]));
                        for _k in 0..node_counts[i+1] {
                            node_weights[i][j].push(0.0);
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

        let flat_data:Vec<u8> = data.into_iter().flat_map(|f| f).collect();
        
        let mut layer: Vec<f64> = flat_data.into_iter().map(|f| f as f64).collect(); 
        let mut next_layer: Vec<f64> = Vec::new();
        
        for (i, layer_weights) in self.node_weights.iter().enumerate() {
            next_layer = vec![0.0; self.node_counts[i + 1]];
            for (j, weights) in layer_weights.iter().enumerate() {
                for (k, weight) in weights.iter().enumerate() {
                    next_layer[k] += layer[j] as f64 * weight;
                }
            }
            next_layer = next_layer.iter().map(|&x| if x >= 0.0 {x} else {0.0}).collect::<Vec<f64>>();
            layer = next_layer;
        }

        let probabilities: Vec<f64> = layer.iter().map(|&f| E.powf(f)).collect();
        let probability = probabilities[label as usize];
        let mut sum = 0.0;
        for p in probabilities {
            sum += p;
        }
        let loss = - (probability / sum).log(E);

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