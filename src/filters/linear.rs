// Função para aplicar uma função linear definida por partes a uma imagem
pub fn with_linear(image_data: &[u8], breakpoints: &[(u8, u8)], slopes: &[f64]) -> Vec<u8> {
    assert_eq!(
        breakpoints.len() + 1,
        slopes.len(),
        "Número incorreto de inclinações"
    );

    let mut corrected_data = Vec::with_capacity(image_data.len());

    for &pixel in image_data {
        let normalized_pixel = pixel as f64 / 255.0; // Normaliza o valor do pixel para o intervalo [0, 1]

        // Encontra o segmento linear correspondente ao valor do pixel
        let mut segment_index = 0;
        for (index, &(breakpoint, _)) in breakpoints.iter().enumerate() {
            if pixel <= breakpoint {
                segment_index = index;
                break;
            }
        }

        // Aplica a função linear definida por partes ao valor do pixel
        let (x0, y0) = if segment_index == 0 {
            (0.0, 0.0)
        } else {
            (
                breakpoints[segment_index - 1].0 as f64 / 255.0,
                breakpoints[segment_index - 1].1 as f64 / 255.0,
            )
        };

        let m = slopes[segment_index];
        let corrected_pixel = (((normalized_pixel - x0) * m + y0) * 255.0) as u8;
        corrected_data.push(corrected_pixel);
    }

    corrected_data
}
