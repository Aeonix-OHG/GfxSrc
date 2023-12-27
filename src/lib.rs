 /*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: lib.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 21 12 2023
 * Modified: 27 12 2023
 * Modified By: Jan Simon Schmitt
 */
mod color;
use std::io::{self, Write};

   

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
    pub fn showpopup(&mut self, popuptitle: String, popuptext: String, color : String){
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
    pub fn set_title(&mut self, title : String, titlecolor : String) {
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
    pub fn addoutline(&mut self, color: String) {
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
    pub fn addinput(&mut self, x: usize, y: usize, promt : String, color: &str) -> String {
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
       let mut app = Screen::new(30, 30, ' '.to_string());
       app.set_title("Testapp".to_owned(), "#FFFFFF".to_owned());
       app.addstring(2, 4, "123456", "#FFFFFF");
       app.addoutline("#FFFFFF".to_string());
       let var1 = app.addinput(2, 6, "==> ".to_owned(), "#ff003c");
       app.addstring(2, 7, &var1, "#32a852");
       app.addstring(2, 7, &var1, "#f6ff00");
       app.print();
       app.updatewindow(50, 30, ' '.to_string());
       app.cls();
       app.set_title("123test".to_owned(), "#FFFFFF".to_owned());
       app.showpopup("test".to_string(), "Lorem test test test test test test test test ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt est ".to_string(), "#FFFFFF".to_string());
       app.print();
   }
}
