use std::cmp::min;

use anyhow::{Context, Result};
use colored::Colorize;
use image::imageops::FilterType;
use reqwest::blocking::get;

const ASCII: [&str; 6] = [
    "$", "@", "%", "&", "#", "*",
];

const MAX_WIDTH: usize = 128;

// To slow
pub fn get_image(url: &str, longest_text: &str) -> Result<Vec<String>> {
    let (w, _) = term_size::dimensions().context("Not able to get width")?;
    let image_width = min(w - format!("         {}", longest_text).len(), MAX_WIDTH);

    let bytes = get(url)?.bytes()?;
    // FIX : improve, the blur is too slow
    // debug without blur : 0.5s
    // debug with blur : 1.28s
    // release without blur : 0.27s
    // release with blur : 0.30s
    // bottleneck here comes from http request
    let img = image::load_from_memory(&bytes)?
        .fast_blur(10.)
        .adjust_contrast(10.);
    let (x, y) = (image_width, image_width * 11 / 25);
    let resized = img.resize_exact(x as u32, y as u32, FilterType::Nearest);
    let lumas = resized.to_luma8();
    let colors = resized.to_rgb8();
    let height = resized.height();
    let width = resized.width();
    let mut ascii = vec![vec![String::new(); width as usize]; height as usize];
    let factor: f32 = (ASCII.len() - 1) as f32 / 0xff as f32;
    for i in 0..height {
        for j in 0..width {
            let luma = lumas.get_pixel(j, i);
            let [r, g, b] = colors.get_pixel(j, i).0;
            let char = ASCII[(luma.0[0] as f32 * factor) as usize];
            ascii[i as usize][j as usize] = char.truecolor(r, g, b).to_string();
        }
    }
    
    Ok(ascii.into_iter().map(|v| v.join("")).collect())
}