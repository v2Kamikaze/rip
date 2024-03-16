pub fn gamma_correction(image_data: &[u8], gamma: f64, c: f64) -> Vec<u8> {
    let mut corrected_data = Vec::with_capacity(image_data.len());

    for &pixel in image_data {
        let normalized_pixel = pixel as f64 / 255.0; // Normaliza o valor do pixel para o intervalo [0, 1]
        let corrected_pixel = (c * normalized_pixel.powf(gamma) * 255.0) as u8;
        corrected_data.push(corrected_pixel);
    }

    corrected_data
}
