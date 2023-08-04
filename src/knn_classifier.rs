use std::collections::HashMap;
use crate::classifier::Classifier;

pub struct KNNClassifier {
    pub k: usize,
    data: Vec<Vec<Vec<u8>>>,
    labels: Vec<u8>,
}

impl KNNClassifier {
    pub fn new(k: usize) -> KNNClassifier {
        let nearest_neighbor = KNNClassifier {
            k,
            data: Vec::new(),
            labels: Vec::new(),
        };
        nearest_neighbor
    }

    pub fn distance(image1: &Vec<Vec<u8>>, image2: &Vec<Vec<u8>>) -> u128 {
        let mut distance: u128 = 0;
        for (row1, row2) in image1.iter().zip( image2.iter()) {
            for (d1, d2) in row1.iter().zip(row2.iter()) {
                distance += (d1.abs_diff(d2.clone()) as u32).pow(2) as u128;
            }
        }
        distance
    }
}

impl Classifier<Vec<Vec<u8>>, u8> for KNNClassifier {
    fn learn(&mut self, data: Vec<Vec<u8>>, label: u8) {
        self.data.push(data);
        self.labels.push(label);
    }

    fn predict(&self, data: &Vec<Vec<u8>>) -> u8 {
        let mut nearest_neighbors: Vec<(u128, u8)> = Vec::new();

        for (i, d) in self.data.iter().enumerate() {
            let distance = KNNClassifier::distance(&data, d);
            let result = nearest_neighbors.binary_search_by(|probe| probe.0.cmp(&distance));
            match result {
                Ok(index) => nearest_neighbors.insert(index, (distance, self.labels[i])),
                Err(index) => nearest_neighbors.insert(index, (distance, self.labels[i]))
            }
        }

        let mut map: HashMap<u8, u64> = HashMap::new();
        let mut max_count: u64 = 0;
        let mut label = 0;
        let smallest = nearest_neighbors[0].0;

        for (i, neighbor) in nearest_neighbors.iter().enumerate() {
            if i >= self.k && neighbor.0 != smallest {
                break;
            }

            let count = map.entry(neighbor.1).and_modify(|counter| *counter += 1).or_insert(1);

            if max_count < *count {
                max_count = *count;
                label = neighbor.1;
            }
        }

        return label;
    }
}