pub fn scale(image_data: &[u8], width: usize, height: usize, scale_factor: f64) -> Vec<u8> {
    let new_width = width;
    let new_height = height;
    let mut scaled_data = vec![0; new_width * new_height * 3];

    for y in 0..new_height {
        for x in 0..new_width {
            // Coordenadas na imagem original
            let src_x = (x as f64 / scale_factor) as usize;
            let src_y = (y as f64 / scale_factor) as usize;

            // Se estiver dentro da imagem original, realiza a interpolação bilinear
            if src_x < width && src_y < height {
                // Coordenadas dos pixels vizinhos na imagem original
                let x0 = src_x;
                let x1 = (src_x + 1).min(width - 1);
                let y0 = src_y;
                let y1 = (src_y + 1).min(height - 1);

                // Frações para interpolação
                let tx = x as f64 / scale_factor - src_x as f64;
                let ty = y as f64 / scale_factor - src_y as f64;

                // Interpolação bilinear
                let interpolated_pixel = interpolate_bilinear(
                    image_data[y0 * width * 3 + x0 * 3],
                    image_data[y0 * width * 3 + x1 * 3],
                    image_data[y1 * width * 3 + x0 * 3],
                    image_data[y1 * width * 3 + x1 * 3],
                    tx,
                    ty,
                );

                // Preenche os dados da imagem escalada
                let index = y * new_width * 3 + x * 3;
                scaled_data[index] = interpolated_pixel[0];
                scaled_data[index + 1] = interpolated_pixel[1];
                scaled_data[index + 2] = interpolated_pixel[2];
            } else {
                // Fora da imagem original, preenche com pixels pretos
                let index = y * new_width * 3 + x * 3;
                scaled_data[index] = 0;
                scaled_data[index + 1] = 0;
                scaled_data[index + 2] = 0;
            }
        }
    }

    scaled_data
}

// Função de interpolação bilinear
fn interpolate_bilinear(p00: u8, p10: u8, p01: u8, p11: u8, tx: f64, ty: f64) -> [u8; 3] {
    let x0 = p00 as f64 * (1.0 - tx) + p10 as f64 * tx;
    let x1 = p01 as f64 * (1.0 - tx) + p11 as f64 * tx;
    let result = x0 * (1.0 - ty) + x1 * ty;
    [result as u8, result as u8, result as u8] // A imagem é em tons de cinza, então todos os canais têm o mesmo valor
}
