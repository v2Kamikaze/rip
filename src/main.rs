use img_proc::{gamma, gauss, image::Image, log, sobel};

fn main() {
    let mut image = Image::from("lua.jpg");

    let log_data = log::logarithmic_transformation(&image.get_data(), 1.0);
    let gamma_data = gamma::gamma_correction(&log_data, 2.2, 1.0);

    image.set_data(gamma_data);

    let gaussian_filter = gauss::kernel(3, 1.4);
    let gauss_data = image.convolve(&gaussian_filter, 3);
    image.set_data(gauss_data);

    let sobel_x = image.convolve(&sobel::HORIZONTAL_KERNEL, 3);
    let sobel_y = image.convolve(&sobel::VERTICAL_KERNEL, 3);
    let sobel_full = sobel::gradient_magnitude(&sobel_x, &sobel_y);

    image.set_data(sobel_full);

    image.save("new_image.jpg");
}
