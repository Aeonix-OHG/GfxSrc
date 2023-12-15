use std::io;

pub struct Screen {
    width: usize,
    height: usize,
    standartchar: char,
    frame: Vec<Vec<char>>,
}

impl Screen {
    pub fn new(width: usize, height: usize, standartchar: char) -> Screen {
        let frame = vec![vec![standartchar; width]; height];
        Screen {width, height, standartchar, frame}
    }
    pub fn print(&self) {
        for line in &self.frame {
            for character in line {
                print!("{}", character);
            }
            println!()
        }
    }
    pub fn addinput(&mut self, x: usize, y: usize) -> String {
        self.addstring(x, y, ">    ");
        println!("{}{}", x, y );
        self.print();
        let mut inputofusr = String::new();
        io::stdin()
            .read_line(&mut inputofusr)
            .expect("Error by reading input");
        println!("{}{}", x + 1, y );
        self.addstring(x + 1, y, &inputofusr.trim().to_string());
        self.print();
        inputofusr.trim().to_string()
    }
    pub fn cls(&mut self) {
        self.frame = vec![vec![self.standartchar; self.width]; self.height];
        
    }
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

#[cfg(test)]
mod tests {
    use crate::Screen;

    #[test]
    fn it_works() {
        let mut app = Screen::new(10, 10, '#');
        app.addstring(2, 2, "test");
        let var1 = app.addinput(2, 3);
        app.addstring(2, 7, &var1);
        app.print();
    }
}
