pub const HORIZONTAL_KERNEL: [f32; 9] = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

pub const VERTICAL_KERNEL: [f32; 9] = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];

pub fn gradient_magnitude(sobel_x: &[u8], sobel_y: &[u8]) -> Vec<u8> {
    // Verifica se os tamanhos dos vetores são iguais
    assert_eq!(sobel_x.len(), sobel_y.len());

    // Vetor para armazenar a magnitude do gradiente como bytes
    let mut magnitude_bytes = Vec::with_capacity(sobel_x.len());

    // Calcula a magnitude do gradiente para cada ponto e converte para bytes
    for i in 0..sobel_x.len() {
        let gx = sobel_x[i] as f32;
        let gy = sobel_y[i] as f32;
        let mag = (gx * gx + gy * gy).sqrt().clamp(0.0, 255.0) as u8;
        magnitude_bytes.push(mag);
    }

    magnitude_bytes
}
