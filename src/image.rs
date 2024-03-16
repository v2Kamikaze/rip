use image::ImageBuffer;

pub struct Image {
    data: Vec<u8>,
    height: u32,
    width: u32,
}

impl From<&str> for Image {
    fn from(path: &str) -> Self {
        let img = image::open(path).unwrap();
        let img_rgb8 = img.as_rgb8().unwrap();
        let mut matrix: Vec<u8> =
            Vec::with_capacity((img_rgb8.width() * img_rgb8.height()) as usize * 3);

        for pixel in img_rgb8.pixels() {
            matrix.push(pixel[0]);
            matrix.push(pixel[1]);
            matrix.push(pixel[2]);
        }

        Image {
            data: matrix,
            height: img_rgb8.height(),
            width: img_rgb8.width(),
        }
    }
}

impl Image {
    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        let width = self.width;
        if x >= width || y >= self.height {
            panic!("Índice fora dos limites da imagem");
        }

        let index = (y as usize * width as usize + x as usize) * 3;
        [self.data[index], self.data[index + 1], self.data[index + 2]]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        let width = self.width;
        if x >= width || y >= self.height {
            panic!("Índice fora dos limites da imagem");
        }

        let index = (y as usize * width as usize + x as usize) * 3;
        self.data[index] = color[0];
        self.data[index + 1] = color[1];
        self.data[index + 2] = color[2];
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

impl Image {
    pub fn save(&self, path: &str) {
        let mut image_buffer: image::RgbImage = ImageBuffer::new(self.width, self.height);
        let mut image_iter = image_buffer.pixels_mut();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y as usize * self.width as usize + x as usize) * 3;
                let pixel = &mut image_iter.next().unwrap();
                pixel[0] = self.data[index];
                pixel[1] = self.data[index + 1];
                pixel[2] = self.data[index + 2];
            }
        }

        image_buffer.save(path).unwrap()
    }
}

impl Image {
    pub fn convolve(&mut self, kernel: &[f32], n: usize) -> Vec<u8> {
        let mut new_data = self.data.clone(); // Criando uma nova cópia dos dados da imagem

        let k_radius = n / 2; // Raio do kernel

        let mut new_pixel = [0.0, 0.0, 0.0];

        // Loop sobre cada pixel da imagem
        for y in 0..self.height {
            for x in 0..self.width {
                // Loop sobre cada canal de cor (R, G, B)
                for c in 0..3 {
                    let mut conv_value = 0.0;

                    // Loop sobre cada pixel do kernel
                    for ky in 0..n {
                        for kx in 0..n {
                            let img_x = x as isize + kx as isize - k_radius as isize;
                            let img_y = y as isize + ky as isize - k_radius as isize;

                            // Verifica se o pixel está dentro da imagem
                            if img_x >= 0
                                && img_x < self.width as isize
                                && img_y >= 0
                                && img_y < self.height as isize
                            {
                                let img_index =
                                    (img_y as usize * self.width as usize + img_x as usize) * 3 + c;
                                let kernel_index = ky * n + kx;
                                conv_value += self.data[img_index] as f32 * kernel[kernel_index];
                            }
                        }
                    }

                    new_pixel[c] = conv_value;
                }

                // Atualiza o pixel na nova imagem
                let index = (y as usize * self.width as usize + x as usize) * 3;
                for c in 0..3 {
                    new_data[index + c] = new_pixel[c] as u8;
                }
            }
        }

        new_data
    }
}
