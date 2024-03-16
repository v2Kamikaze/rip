// Função para aplicar uma função linear definida por partes a uma imagem
pub fn linear_parts(image_data: &[u8], segments: &[(f64, f64)], c: f64) -> Vec<u8> {
    let mut transformed_data = Vec::with_capacity(image_data.len());

    for &pixel in image_data {
        let normalized_pixel = pixel as f64 / 255.0; // Normaliza o valor do pixel para o intervalo [0, 1]
        let mut transformed_pixel = 0.0;

        // Encontra o segmento correspondente para o valor do pixel
        let mut prev_intensity = 0.0;
        let mut prev_output = 0.0;
        for &(intensity, output) in segments {
            if normalized_pixel <= intensity {
                transformed_pixel = prev_output
                    + (normalized_pixel - prev_intensity) * (output - prev_output)
                        / (intensity - prev_intensity);
                break;
            }
            prev_intensity = intensity;
            prev_output = output;
        }

        transformed_pixel *= c * 255.0;
        transformed_data.push(transformed_pixel as u8);
    }

    transformed_data
}
