/*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: color.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 25 12 2023
 * Modified: 25 12 2023
 * Modified By: Jan Simon Schmitt
 */

// Creade Mod For colors
pub mod color {
    // Conver Hex to colors
    pub fn get_color(hex_color: &str) -> Result<String, std::num::ParseIntError> {
        let r = i64::from_str_radix(&hex_color[1..3], 16)?;
        let g = i64::from_str_radix(&hex_color[3..5], 16)?;
        let b = i64::from_str_radix(&hex_color[5..7], 16)?;
 
        let closest_ansi = closest_ansi_color(r, g, b);
 
        Ok(format!("\x1B[38;5;{}m", closest_ansi))
    }
    // Convert Colors to Ansi-escape codes 
    fn closest_ansi_color(r: i64, g: i64, b: i64) -> u8 {
        if r == g && g == b {
            if r < 85 { 16 } else if r > 255 - 85 { 231 } else { ((r + 15) / 36 * 36).try_into().unwrap() }
        } else {
            let ansi_colors = [
                [0, 0, 0], [128, 0, 0], [0, 128, 0], [128, 128, 0], [0, 0, 128], [128, 0, 128], [0, 128, 128], [192, 192, 192],
                [128, 128, 128], [255, 0, 0], [0, 255, 0], [255, 255, 0], [0, 0, 255], [255, 0, 255], [0, 255, 255], [255, 255, 255]
            ];
            let mut closest = 0;
            let mut min_distance = std::f64::MAX;
            for (i, color) in ansi_colors.iter().enumerate() {
                let distance = (r - color[0]) as f64 * (r - color[0]) as f64
                    + (g - color[1]) as f64 * (g - color[1]) as f64
                    + (b - color[2]) as f64 * (b - color[2]) as f64;
                if distance < min_distance {
                    closest = i;
                    min_distance = distance;
                }
            }
            closest as u8
        }
     }
}