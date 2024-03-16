pub fn logarithmic_transformation(image_data: &[u8], c: f64) -> Vec<u8> {
    let mut transformed_data = Vec::with_capacity(image_data.len());

    for &pixel in image_data {
        let normalized_pixel = pixel as f64 / 255.0; // Normaliza o valor do pixel para o intervalo [0, 1]
        let transformed_pixel = (c * (1.0 + normalized_pixel).ln() * 255.0) as u8;
        transformed_data.push(transformed_pixel);
    }

    transformed_data
}
