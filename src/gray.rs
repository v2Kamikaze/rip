pub fn grayscale(image_data: &[u8]) -> Vec<u8> {
    let mut grayscale_data = Vec::with_capacity(image_data.len());

    // Iterar pelos pixels e calcular a média dos valores dos canais de cor
    for pixel in image_data.chunks(3) {
        let red = pixel[0] as f32;
        let green = pixel[1] as f32;
        let blue = pixel[2] as f32;

        let grayscale_value = ((red + green + blue) / 3.0) as u8;

        grayscale_data.push(grayscale_value);
        grayscale_data.push(grayscale_value);
        grayscale_data.push(grayscale_value);
    }

    grayscale_data
}
