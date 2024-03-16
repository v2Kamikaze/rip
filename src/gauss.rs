/// Função para calcular o valor de uma distribuição gaussiana 2D em um ponto (x, y)
fn gaussian(x: f32, y: f32, sigma: f32) -> f32 {
    let coefficient = 1.0 / (2.0 * std::f32::consts::PI * sigma * sigma);
    let exponent = -((x * x + y * y) / (2.0 * sigma * sigma));
    coefficient * exponent.exp()
}

// Função para criar um filtro gaussiano com uma determinada largura e desvio padrão
pub fn kernel(size: usize, sigma: f32) -> Vec<f32> {
    let mut kernel = vec![0.0; size * size];
    let center = size as f32 / 2.0;

    // Calcula os valores do kernel gaussiano
    for y in 0..size {
        for x in 0..size {
            let distance_x = (x as f32 - center) / size as f32;
            let distance_y = (y as f32 - center) / size as f32;
            let gaussian_value = gaussian(distance_x, distance_y, sigma);
            kernel[y * size + x] = gaussian_value;
        }
    }

    // Normaliza o kernel para que a soma dos valores seja igual a 1
    let sum: f32 = kernel.iter().sum();
    kernel.iter_mut().for_each(|x| *x /= sum);

    kernel
}
