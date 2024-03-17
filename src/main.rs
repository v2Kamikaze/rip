use img_proc::filters::{gamma, gauss, gray, linear, log, sobel};
use img_proc::image::image::Image;
use img_proc::transform::{rotate, scale, translate};

const BREAKPOINTS: [(u8, u8); 2] = [(100, 50), (200, 150)];
const SLOPES: [f64; 3] = [0.5, 1.0, 0.75];

fn main() {
    let mut image = Image::from("image2.jpg");

    let linear_data = linear::with_linear(&image.get_data(), &BREAKPOINTS, &SLOPES);
    let log_data = log::logarithmic_transformation(&linear_data, 1.0);
    let gamma_data = gamma::gamma_correction(&log_data, 1.0, 1.0);
    image.set_data(gamma_data);

    let gaussian_filter = gauss::kernel(3, 1.0);
    let gauss_data = image.convolve(&gaussian_filter, 3);
    let gray_data = gray::grayscale(&gauss_data);
    image.set_data(gray_data);

    let sobel_x = image.convolve(&sobel::HORIZONTAL_KERNEL, 3);
    let sobel_y = image.convolve(&sobel::VERTICAL_KERNEL, 3);
    let sobel_full = sobel::gradient_magnitude(&sobel_x, &sobel_y);
    let translate_data = translate::translate(
        &sobel_full,
        image.width() as usize,
        image.height() as usize,
        0,
        0,
    );

    let rotate_data = rotate::rotate(
        &translate_data,
        image.width() as usize,
        image.height() as usize,
        45.0,
    );

    let scale_data = scale::scale(
        &rotate_data,
        image.width() as usize,
        image.height() as usize,
        0.5,
    );

    image.set_data(scale_data);

    image.save("new_image.png");
}
