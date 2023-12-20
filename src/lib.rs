/*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: lib.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 11 12 2023
 * Modified: 20 12 2023
 * Modified By: Jan Simon Schmitt
 */
use std::io::{self, Write};

// Implementing the Screen Structure
pub struct Screen {
    width: usize,
    height: usize,
    standartchar: char,
    frame: Vec<Vec<char>>,
}

// Implementing the screen
impl Screen {
    // Creating a new Screen
    pub fn new(width: usize, height: usize, standartchar: char) -> Screen {
        let frame = vec![vec![standartchar; width]; height];
        Screen {width, height, standartchar, frame}
    }
    // print out the screen
    pub fn print(&self) {
        for line in &self.frame {
            for character in line {
                print!("{}", character);
            }
            println!()
        }
    }
    // adds an input filed to the screen
    pub fn addinput(&mut self, x: usize, y: usize, promt : String) -> String {
        let xp = promt.len();
        self.addstring(x, y, &promt);
        self.print();
        print!("{}", promt);
        io::stdout().flush().unwrap();
        let mut inputofusr = String::new();
        io::stdin()
            .read_line(&mut inputofusr)
            .expect("Error by reading input");
        self.addstring(x + xp, y, &inputofusr.trim().to_string());
        self.print();
        inputofusr.trim().to_string()
    }
    // clears the screen
    pub fn cls(&mut self) {
        self.frame = vec![vec![self.standartchar; self.width]; self.height];
        
    }
    // adds a sting to the screen
    pub fn addstring(&mut self, x: usize, y: usize, text: &str) {
        if !(x + text.len() > self.width) && y <= self.height{
            let textlist = text.chars();
            if let Some(row) = self.frame.get_mut(y) {
                for (i, c) in textlist.enumerate() {
                    if let Some(cell) = row.get_mut(x + i) {
                        *cell = c;
                    }
                }
            }
        }
    }
}

// Tests all funktions

#[cfg(test)]
mod tests {
    use crate::Screen;

    #[test]
    fn it_works() {
        let mut app = Screen::new(30, 30, ' ');
        app.addstring(2, 2, "123456");
        let var1 = app.addinput(2, 3, "==> ".to_owned());
        app.addstring(2, 7, &var1);
        app.print();
    }
}
