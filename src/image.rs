pub struct Image {
    pub data: Vec<Vec<u8>>,
    pub label: u8
}

impl Image {
    pub fn new(row_size: u32, col_size: u32, mut bytes: Vec<u8>, label: u8) -> Image {
        let mut data: Vec<Vec<u8>> = Vec::new();

        for i in 0..col_size as usize {
            data.push(Vec::new());
            data[i as usize].extend(bytes.drain(0..row_size as usize));
        }

        Image { data, label }
    }

    pub fn label(&mut self, label: u8) {
        self.label = label;
    }
}