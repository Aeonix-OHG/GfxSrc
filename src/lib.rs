struct Screen {
    width: usize,
    height: usize,
    standartchar: char,
    frame: Vec<Vec<char>>,
}

impl Screen {
    fn new(width: usize, height: usize, standartchar: char) -> Screen {
        let frame = vec![vec![standartchar; width]; height];
        Screen {width, height, standartchar, frame}
    }
    fn print(&self) {
        for line in &self.frame {
            for character in line {
                print!("{}", character);
            }
            println!()
        }
    }
    fn cls(&mut self) {
        self.frame = vec![vec![self.standartchar; self.width]; self.height];
        
    }
    fn addstring(&mut self, x: usize, y: usize, text: &str) {
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
        let mut app = Screen::new(20, 20, '/');
        app.print();
        app.addstring(10, 10, "test");
        app.print();
        app.cls();
        app.print();
    }
}
