mod image;

use std::{fs::File, io::{BufReader, Read}, rc::Rc};
use crate::image::Image;

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(File::open("./data/mnist_train/images").unwrap());
    let mut meta_data = vec![0; 16];

    let mut image_count: u32 = 0;

    let mut row_size: u32 = 0;
    let mut col_size: u32 = 0; 

    let mut temp_u32 = 0;

    reader.read_exact(&mut meta_data).unwrap();
    for (i, byte) in meta_data.iter().enumerate() {
        

        if i < 4 {
            continue;
        }

        if i >=4 && i < 8 {
            temp_u32 += (*byte as u32)<<((3-i%4)*8);
            if i == 7 {
                image_count = temp_u32;
                temp_u32 = 0;
            }
        }

        if i >=8 && i < 12 {
            temp_u32 += (*byte as u32)<<((3-i%4)*8);
            if i == 11 {
                row_size = temp_u32;
                temp_u32 = 0;
            }
        }

        if i >=12 && i < 16 {
            temp_u32 += (*byte as u32)<<((3-i%4)*8);
            if i == 15 {
                col_size = temp_u32;
                temp_u32 = 0;
            }
        }
    }

    let mut bytes: Vec<Vec<u8>> = Vec::new();
    for i in 0..image_count {
        bytes.push(vec![0; row_size as usize * col_size as usize]);
        reader.read_exact(&mut bytes[i as usize]).unwrap();
    }
    
    let mut images: Vec<Image> = Vec::new();

    for _ in 0..image_count {
        let image = Image::new(row_size, col_size, bytes.remove(0)); 
        images.push(image);
    }

    println!("{}", images.len());

}
