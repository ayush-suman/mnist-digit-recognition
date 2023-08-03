use std::{fs::File, io::{BufReader, Read}, cmp::min};

use crate::image::Image;

pub fn load_training_images() -> Vec<Image> {
    let mut reader = BufReader::new(File::open("./data/mnist_train/images").unwrap());

    let mut meta_data = vec![0; 16];

    let mut image_count: u32 = 0;

    let mut row_size: u32 = 0;
    let mut col_size: u32 = 0; 

    reader.read_exact(&mut meta_data).unwrap();

    for i in 4..8 as usize {
        image_count += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    for i in 8..12 as usize {
        row_size += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    for i in 8..12 as usize {
        col_size += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    let mut bytes: Vec<Vec<u8>> = Vec::new();
    for i in 0..image_count {
        bytes.push(vec![0; row_size as usize * col_size as usize]);
        reader.read_exact(&mut bytes[i as usize]).unwrap();
    }
    
    let mut images: Vec<Image> = Vec::new();

    reader = BufReader::new(File::open("./data/mnist_train/labels").unwrap());

    meta_data = vec![0; 8];

    let mut label_count: u32 = 0;

    reader.read_exact(&mut meta_data).unwrap();

    for i in 4..8 as usize {
        label_count += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    let mut labels: Vec<u8> = vec![0; label_count as usize]; 
    reader.read_exact(&mut labels).unwrap();
    
    let count = min(image_count, label_count);

    for _ in 0..count {
        let image= Image::new(row_size, col_size, bytes.remove(0), labels.remove(0)); 
        images.push(image);
    }

    images
}


pub fn load_test_images() -> Vec<Image> {
    let mut reader = BufReader::new(File::open("./data/mnist_test/images").unwrap());

    let mut meta_data = vec![0; 16];

    let mut image_count: u32 = 0;

    let mut row_size: u32 = 0;
    let mut col_size: u32 = 0; 

    reader.read_exact(&mut meta_data).unwrap();

    for i in 4..8 as usize {
        image_count += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    for i in 8..12 as usize {
        row_size += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    for i in 8..12 as usize {
        col_size += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    let mut bytes: Vec<Vec<u8>> = Vec::new();
    for i in 0..image_count {
        bytes.push(vec![0; row_size as usize * col_size as usize]);
        reader.read_exact(&mut bytes[i as usize]).unwrap();
    }
    
    let mut images: Vec<Image> = Vec::new();

    reader = BufReader::new(File::open("./data/mnist_test/labels").unwrap());

    meta_data = vec![0; 8];

    let mut label_count: u32 = 0;

    reader.read_exact(&mut meta_data).unwrap();

    for i in 4..8 as usize {
        label_count += (meta_data[i] as u32)<<((3-i%4)*8);
    }

    let mut labels: Vec<u8> = vec![0; label_count as usize]; 
    reader.read_exact(&mut labels).unwrap();
    
    let count = min(image_count, label_count);

    for _ in 0..count {
        let image= Image::new(row_size, col_size, bytes.remove(0), labels.remove(0)); 
        images.push(image);
    }

    images
}
