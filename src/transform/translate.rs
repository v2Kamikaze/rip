pub fn translate(
    image_data: &[u8],
    width: usize,
    height: usize,
    x_offset: isize,
    y_offset: isize,
) -> Vec<u8> {
    let mut translated_data = vec![0; image_data.len()];

    for y in 0..height {
        for x in 0..width {
            let new_x = (x as isize + x_offset).clamp(0, width as isize - 1) as usize;
            let new_y = (y as isize + y_offset).clamp(0, height as isize - 1) as usize;

            let original_index = y * width * 3 + x * 3;
            let translated_index = new_y * width * 3 + new_x * 3;

            // Copia o pixel da posição original para a posição traduzida
            translated_data[translated_index] = image_data[original_index];
            translated_data[translated_index + 1] = image_data[original_index + 1];
            translated_data[translated_index + 2] = image_data[original_index + 2];
        }
    }

    translated_data
}
