use rand::Rng;
use std::f32::consts::PI;
use image::{GrayImage, Luma};
use std::time::Instant;

fn main() {
    //start timer
    let start = Instant::now();

    perlin_noise();

    // print out how long it took
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn perlin_noise() {
    let height = 1000;
    let width = 1000;
    let resolution = 0.03f32;
    let gridpoint_width = (width as f32 * resolution) as usize;
    let gridpoint_height = (height as f32 * resolution) as usize;
    let mut rng = rand::rng();
    let mut img = GrayImage::new(width, height);

    let mut gridpoint_vec: Vec<Vec<[f32; 2]>> = vec![vec![[0.0, 0.0]; gridpoint_width + 1]; gridpoint_height + 1];

    // creating a grid of points with a random normalized vector
    for gy in 0..=gridpoint_height {
        for gx in 0..=gridpoint_width {

            let theta = rng.random_range(0.0..(2.0 * PI));
            gridpoint_vec[gy][gx] = [
                theta.cos(),
                theta.sin(),
            ]
        }
    }

    for py in 0..height {
        for px in 0..width {

            // increment sample point position for rendering
            let sample_point_x = px as f32 * resolution;
            let sample_point_y = py as f32 * resolution;

            // define points in local space
            let local_sample_point_x = sample_point_x % 1.0;
            let local_sample_point_y = sample_point_y % 1.0;
            let local_A_x = sample_point_x.floor() as usize;
            let local_A_y = sample_point_y.floor() as usize;
            let local_B_x = (local_A_x + 1).min(gridpoint_width);
            let local_B_y = local_A_y;
            let local_C_x = (local_A_x + 1).min(gridpoint_width);
            let local_C_y = (local_A_y + 1).min(gridpoint_height);
            let local_D_x = local_A_x;
            let local_D_y = (local_A_y + 1).min(gridpoint_height);

            // calculation vector from gridpoint to sample point
            let A_vec_distance = [local_sample_point_x - 0.0, local_sample_point_y - 0.0];
            let B_vec_distance = [local_sample_point_x - 1.0, local_sample_point_y - 0.0];
            let C_vec_distance = [local_sample_point_x - 1.0, local_sample_point_y - 1.0];
            let D_vec_distance = [local_sample_point_x - 0.0, local_sample_point_y - 1.0];

            // calculationg the dot product from both vectors at each gridpoint
            let A_vec_dot = (A_vec_distance[0] * gridpoint_vec[local_A_y][local_A_x][0]) + (A_vec_distance[1] * gridpoint_vec[local_A_y][local_A_x][1]);
            let B_vec_dot = (B_vec_distance[0] * gridpoint_vec[local_B_y][local_B_x][0]) + (B_vec_distance[1] * gridpoint_vec[local_B_y][local_B_x][1]);
            let C_vec_dot = (C_vec_distance[0] * gridpoint_vec[local_C_y][local_C_x][0]) + (C_vec_distance[1] * gridpoint_vec[local_C_y][local_C_x][1]);
            let D_vec_dot = (D_vec_distance[0] * gridpoint_vec[local_D_y][local_D_x][0]) + (D_vec_distance[1] * gridpoint_vec[local_D_y][local_D_x][1]);

            // apply smoothstep to the local coordinates
            let smooth_x = smoothstep(local_sample_point_x);
            let smooth_y = smoothstep(local_sample_point_y);

            // calculation the Interpolation using smoothed values
            let lerp_A_B = A_vec_dot + smooth_x * (B_vec_dot - A_vec_dot);
            let lerp_C_D = D_vec_dot + smooth_x * (C_vec_dot - D_vec_dot);
            let lerp_AB_CD = lerp_A_B + smooth_y * (lerp_C_D - lerp_A_B);

            // convert pixel value and filling the buffer
            let value = ((lerp_AB_CD + 1.0) * 0.5 * 255.0) as u8;
            img.put_pixel(px, py, Luma([value]));
        }
        let progress = if py != 0 {
            py as f32 / height as f32 * 100.0
        } else {
            0.0
        };
        println!("Progress: {}%", progress.round());
    }
    img.save("perlin_noise.png").unwrap();
}

// Add this smoothstep function
fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}