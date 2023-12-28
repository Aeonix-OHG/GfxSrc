 /*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: lib.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 21 12 2023
 * Modified: 28 12 2023
 * Modified By: Jan Simon Schmitt
 */
mod color;
use std::io::{self, Write};
use std::fs;

   

// Implementing the Screen Structure
pub struct Screen {
    width: usize,
    height: usize,
    standartchar: String,
    frame: Vec<Vec<String>>,
   }
   
impl Screen {
    // Creating a new Screen
    pub fn new(width: usize, height: usize, standartchar: String) -> Screen {
        let frame = vec![vec![standartchar.clone(); width]; height];
        Screen {width, height, standartchar, frame}
    }

    // Updating the variables of the app
    pub fn updatewindow(&mut self, width: usize, height:usize, standartchar: String) {
        self.width = width;
        self.height = height;
        self.standartchar = standartchar;
    }

    // adds a simple popup
    pub fn showpopup(&mut self, popuptitle: &str, popuptext: &str, color : &str){
        let minline = self.height / 3;
        let maxline = (self.height / 3) *2;
        if !(maxline-minline<5) {
            for i in 1..self.width-1 {
                self.addstring(i, minline, "-", &color);
                self.addstring(i, minline+2, "-", &color);
                self.addstring(i, maxline, "-", &color);
            }
            for i in minline+1..maxline {
                self.addstring(0, i, "|", &color);
                self.addstring(self.width-1, i, "|", &color);
            }
            self.addstring((((self.width-2)-popuptitle.len())/2)+1, minline+1, &popuptitle, &color);
            if !(popuptext.len()>=self.width-2) {
                self.addstring((((self.width-2)-popuptext.len())/2)+1, minline+3, &popuptext, &color);
            } else {
                let poptextvec: Vec<&str> = popuptext.split_whitespace().collect();
                let numlines = maxline - minline - 4;
                let numperline: usize = poptextvec.len()/numlines;
                let mut counter: usize = 0;
                for i in 0..numlines+1 {
                    let mut outstr = "".to_string();
                    for _s in 0..numperline {
                        counter = counter + 1;
                        outstr = outstr + " " + poptextvec[counter]; 
                    }
                    self.addstring((((self.width-2)-outstr.len())/2)+1, minline + 3 + i, &outstr, &color);
                }
            }
        }
    }

    // setting the title of the window
    pub fn set_title(&mut self, title : &str, titlecolor : &str) {
        if !(title.len() + 3 > self.width){
           let x: f64 = ((self.width-title.len())/2) as f64;
           let x: usize = (x.floor()) as usize;
           self.addstring(x, 0, &title, &titlecolor);
           self.addstring(0, 1, &"=".repeat(self.width), &titlecolor);
        }
       }
   
  
   // print out the screen
    pub fn print(&self) {
        let mut framestr = String::new();
        for line in &self.frame {
            for character in line {
                framestr = framestr + character
            }
            framestr = framestr + "\n"
        }
        std::io::stdout().flush().unwrap();
        println!("\x1B[2J\x1B[1;1H\n{}", framestr);
    }
    
    // adds an outline for the window
    pub fn addoutline(&mut self, color: &str) {
        for i in 0..(self.height -1) {
            self.addstring(0, i +1, "|", &color);
            self.addstring(self.width-1, i +1, "|", &color);
        }
        for i in 0..(self.width -2) {
            self.addstring(i+1, 1, "-", &color);
            self.addstring(i+1, self.height-1, "-", &color);
        }
    }

   // adds an input field to the screen
    pub fn addinput(&mut self, x: usize, y: usize, promt : &str, color: &str) -> String {
        let xp = promt.len();
        self.addstring(x, y, &promt, color);
        self.print();
        print!("{}", promt);
        io::stdout().flush().unwrap();
        let mut inputofusr = String::new();
        io::stdin()
            .read_line(&mut inputofusr)
            .expect("Error by reading input");
        self.addstring(x + xp, y, &inputofusr.trim(), color);
        self.print();
        std::io::stdout().flush().unwrap();
        inputofusr
    }
 

   // clears the screen
    pub fn cls(&mut self) {
        self.frame = vec![vec![self.standartchar.clone(); self.width]; self.height];
    }

    // prints .npf picture files
    pub fn addpic(&mut self, xw : usize, yh: usize, path: &str) -> Result<(), std::io::Error> {
        let content = fs::read_to_string(path)?;
        let mut lines: Vec<Vec<String>> = Vec::new();
        for line in content.lines() {
            let mut pixels: Vec<String> = Vec::new();
            for pixel in line.split('p') {
                let mut color = String::new();
                for part in pixel.split('s') {
                    if !part.is_empty() {
                      match part.parse::<u8>() {
                          Ok(val) => color += &format!("{:02X}", val),
                          Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
                      }
                    }
                }
                pixels.push(color);
            }
            lines.push(pixels);
        }
      
        // Add the colors to the Screen structure
        for (y, line) in lines.iter().enumerate() {
            for (x, pixel) in line.iter().enumerate() {
                let color = "#".to_string() + pixel;
                if color.len() >= 7 {
                    self.addstring(x+ xw, y+yh, "#", &color);
                }
            }
        }
      
        Ok(())
      }

   // adds a string to the screen
   pub fn addstring(&mut self, x: usize, y: usize, text: &str, color: &str) {
    match color::color::get_color(color) {
        Ok(ansi_color) => {
            if x + text.len() <= self.width && y < self.height {
                let textlist = text.chars();
                if let Some(row) = self.frame.get_mut(y) {
                   for (i, c) in textlist.enumerate() {
                       if let Some(cell) = row.get_mut(x + i) {
                           *cell = format!("{}{}", ansi_color, c);
                       }
                   }
                }
            }
        },
        Err(err) => {
            println!("Failed to parse color: {}", err);
        }
    }
 }
}
// Tests all functions
#[cfg(test)]
mod tests {
   use crate::Screen;

   #[test]
   fn it_works() {
        let mut screen = Screen::new(80, 30, ' '.to_string());
        screen.addpic(3, 3, "testpic.npf").unwrap();
        screen.print();
       //let mut app = Screen::new(30, 30, ' '.to_string());
       //app.set_title("Testapp", "#FFFFFF");
       //app.addstring(2, 4, "123456", "#FFFFFF");
       //app.addoutline("#FFFFFF");
       //let var1 = app.addinput(2, 6, "==> ", "#ff003c");
       //app.addstring(2, 7, &var1, "#32a852");
       //app.addstring(2, 7, &var1, "#f6ff00");
       //app.print();
       //app.updatewindow(50, 30, ' '.to_string());
       //app.cls();
       //app.set_title("123test", "#FFFFFF");
       //app.showpopup("test", "Lorem test test test test test test test test ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt est ", "#FFFFFF");
       //app.print();
   }
}
