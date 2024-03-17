use std::f64::consts::PI;

// Função para rotacionar uma imagem em um determinado ângulo em graus
pub fn rotate(image_data: &[u8], width: usize, height: usize, angle_degrees: f64) -> Vec<u8> {
    let mut rotated_data = vec![0; image_data.len()];

    // Convertendo o ângulo de graus para radianos
    let angle_radians = angle_degrees * PI / 180.0;

    // Calculando os valores de seno e cosseno do ângulo
    let sin_theta = angle_radians.sin();
    let cos_theta = angle_radians.cos();

    // Calculando as coordenadas do centro da imagem
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    // Iterando sobre todos os pixels na imagem original
    for y in 0..height {
        for x in 0..width {
            // Calculando as coordenadas do pixel após a rotação
            let translated_x = x as f64 - center_x;
            let translated_y = y as f64 - center_y;
            let rotated_x = translated_x * cos_theta - translated_y * sin_theta + center_x;
            let rotated_y = translated_x * sin_theta + translated_y * cos_theta + center_y;

            // Arredondando para as coordenadas do pixel mais próximo
            let rotated_x_rounded = rotated_x.round() as isize;
            let rotated_y_rounded = rotated_y.round() as isize;

            // Verificando se as coordenadas rotacionadas estão dentro dos limites da imagem original
            if rotated_x_rounded >= 0
                && rotated_x_rounded < width as isize
                && rotated_y_rounded >= 0
                && rotated_y_rounded < height as isize
            {
                // Copiando o pixel da posição original para a posição rotacionada
                let original_index = y * width * 3 + x * 3;
                let rotated_index =
                    rotated_y_rounded as usize * width * 3 + rotated_x_rounded as usize * 3;
                rotated_data[rotated_index] = image_data[original_index];
                rotated_data[rotated_index + 1] = image_data[original_index + 1];
                rotated_data[rotated_index + 2] = image_data[original_index + 2];
            }
        }
    }

    rotated_data
}
