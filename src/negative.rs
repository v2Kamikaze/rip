pub fn to_negative(data: &[u8]) -> Vec<u8> {
    let mut negative_data = Vec::with_capacity(data.len());

    // Iterar pelos pixels e inverter os valores de cada canal de cor
    for pixel in data.chunks(3) {
        let red = 255 - pixel[0];
        let green = 255 - pixel[1];
        let blue = 255 - pixel[2];

        negative_data.push(red);
        negative_data.push(green);
        negative_data.push(blue);
    }

    negative_data
}
