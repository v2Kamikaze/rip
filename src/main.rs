use img_proc::{gamma, gauss, gray, image::Image, log, sobel};

fn main() {
    let mut image = Image::from("image2.jpg");

    let log_data = log::logarithmic_transformation(&image.get_data(), 1.0);
    let gamma_data = gamma::gamma_correction(&log_data, 1.0, 1.0);
    image.set_data(gamma_data);

    let gaussian_filter = gauss::kernel(6, 1.0);
    let gauss_data = image.convolve(&gaussian_filter, 3);
    let gray_data = gray::grayscale(&gauss_data);
    image.set_data(gray_data);

    let sobel_x = image.convolve(&sobel::HORIZONTAL_KERNEL, 3);
    let sobel_y = image.convolve(&sobel::VERTICAL_KERNEL, 3);
    let sobel_full = sobel::gradient_magnitude(&sobel_x, &sobel_y);

    image.set_data(sobel_full);

    image.save("new_image.png");
}
