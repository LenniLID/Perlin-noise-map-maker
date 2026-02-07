use rand::Rng;
use std::f32::consts::PI;
use image::{GrayImage, Luma};

fn main() {
    perlin_noise()
}

fn perlin_noise() {
    let height = 1000;
    let width = 1000;
    let res_divisor_y = 1.0 / height as f32;
    let res_divisor_x = 1.0 / width as f32;
    let mut sample_point_x = 0.0;
    let mut sample_point_y = 0.0;
    let mut rng = rand::rng();
    let mut img = GrayImage::new(width, height);

    // creating random normalized vectors for grid points
    let A_theta: f32 = rng.random_range(0.0..(2.0 * PI));
    let B_theta: f32 = rng.random_range(0.0..(2.0 * PI));
    let C_theta: f32 = rng.random_range(0.0..(2.0 * PI));
    let D_theta: f32 = rng.random_range(0.0..(2.0 * PI));

    for y in 0..height {
        for x in 0..width {

            // defining gridpoints position
            let A_x = 0; let A_y = 0;
            let B_x = 0; let B_y = 1;
            let C_x = 1; let C_y = 1;
            let D_x = 0; let D_y = 1;

            // converting random normalized vector in x, y coordinates
            let A_vec_x = A_theta.cos();
            let A_vec_y = A_theta.sin();
            let B_vec_x = B_theta.cos();
            let B_vec_y = B_theta.sin();
            let C_vec_x = C_theta.cos();
            let C_vec_y = C_theta.sin();
            let D_vec_x = D_theta.cos();
            let D_vec_y = D_theta.sin();

            // increment sample point position for rendering
            sample_point_x = x as f32 * res_divisor_x;
            sample_point_y = y as f32 * res_divisor_y;

            // calculation vector from gridpoint to sample point
            let A_vec_P_x = sample_point_x - A_x as f32;
            let A_vec_P_y = sample_point_y - A_y as f32;
            let B_vec_P_x = sample_point_x - B_x as f32;
            let B_vec_P_y = sample_point_y - B_y as f32;
            let C_vec_P_x = sample_point_x - C_x as f32;
            let C_vec_P_y = sample_point_y - C_y as f32;
            let D_vec_P_x = sample_point_x - D_x as f32;
            let D_vec_P_y = sample_point_y - D_y as f32;

            // calculationg the dot product from both vectors at each gridpoint
            let A_vec_dot = (A_vec_P_x * A_vec_x) + (A_vec_P_y * A_vec_y);
            let B_vec_dot = (B_vec_P_x * B_vec_x) + (B_vec_P_y * B_vec_y);
            let C_vec_dot = (C_vec_P_x * C_vec_x) + (C_vec_P_y * C_vec_y);
            let D_vec_dot = (D_vec_P_x * D_vec_x) + (D_vec_P_y * D_vec_y);

            // calculation the Interpolation
            let lerp_A_B = A_vec_dot + sample_point_x * (B_vec_dot - A_vec_dot);
            let lerp_C_D = C_vec_dot + sample_point_x * (D_vec_dot - C_vec_dot);
            let lerp_AB_CD = lerp_A_B + sample_point_y * (lerp_C_D - lerp_A_B);

            // convert pixel value and filling the buffer
            let value = ((lerp_AB_CD + 1.0) * 0.5 * 255.0) as u8;
            img.put_pixel(x, y, Luma([value]));
        }
    }
    img.save("perlin_noise.png").unwrap();
}