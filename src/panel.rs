use crate::document;
use crate::cursor;
use crate::graphics::Transformed;

pub struct Panel {
    pub document: document::Document,
    size: [f64; 2],
    cursor: cursor::Cursor,
    position: [f64; 2],
    lines: Vec<LogicalLine>,
}

// Strings that end in a new line
struct LogicalLine {
    pub text: String,
}

const FONT_SIZE: u32 = 75;
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

impl Panel {
    pub fn new() -> Panel{
        Panel{
            document: document::Document::new(),
            size: [500.0, 500.0],
            cursor: cursor::Cursor::new(),
            position: [0.0, 0.0],
            lines: vec!(LogicalLine::new()),
        }
    }

    pub fn add_text(&mut self, text: &str){
        let cursor_position = &self.cursor.position;
        let logical_line = self.lines.get_mut(cursor_position.line_num as usize).unwrap();
        logical_line.text.push_str(text);
    }

    pub fn remove_text(&mut self, amount: usize){
        let cursor_position = &self.cursor.position;
        let logical_line = self.lines.get_mut(cursor_position.line_num as usize).unwrap();
        for _ in [..amount]{
            logical_line.text.pop();
        }
    }

    pub fn draw<G: graphics::Graphics, C>(&self, glyphs: &mut C, c: graphics::Context, g: &mut G)
    where C: graphics::CharacterCache<Texture = G::Texture>{
        let rect = graphics::Rectangle::new(graphics::color::BLACK);
        rect.draw([self.position[0], self.position[1], self.size[0], self.size[1]], &graphics::DrawState::new_alpha(), c.transform, g);


        for logical_line in self.lines.iter() {
            let physical_lines = logical_line.get_physical_lines(glyphs, FONT_SIZE, self.size[0]);
            for (i, physical_line) in physical_lines.iter().enumerate() {
                
                let text = graphics::text::Text::new_color(RED, FONT_SIZE);
                // Although a line is 4/3 * FONT_SIZE tall
                // the text seems to be drawn such that (0.0, 0.0)
                // refers to aligning the line on ruled paper with (0.0, 0.0)
                // so we need to offset it down by FONT_SIZE so
                // the top of the line aligns with the bottom 
                // of the next line or the top of the window
                let trans = c
                    .transform
                    .trans(0.0, FONT_SIZE as f64 + FONT_SIZE as f64 * 4.0/3.0 * (i as f64));

                if let Err(_) = text.draw(physical_line, glyphs, &graphics::DrawState::new_alpha(), trans, g){
                    panic!("Couldn't render text for some reason");
                }
            }
        }
    }

}

impl LogicalLine {
    pub fn new() -> LogicalLine {
        LogicalLine{
            text: String::new()
        }
    }

    fn get_physical_lines<C: graphics::CharacterCache>(&self, glyphs: &mut C, font_size: u32, width: f64) -> Vec<&str> {
        let mut physical_lines = vec!();

        // Because destructuring on assignment is currently unstable
        // we do it like this https://github.com/rust-lang/rust/issues/71126
        let mut tup = self.get_physical_line(0, glyphs, font_size, width); 
        physical_lines.push(tup.1);

        while tup.0 < self.text.chars().count() {
            tup = self.get_physical_line(tup.0, glyphs, font_size, width);
            physical_lines.push(tup.1);
        }

        physical_lines
    }

    fn get_physical_line <C: graphics::CharacterCache>(&self, start: usize, glyphs: &mut C, font_size: u32, width: f64) -> (usize, &str) {
        let mut line = String::new();
        let chars = self.text.chars();
        let mut chars = chars.skip(start);
        let mut end = start;

        while let Some(c) = chars.next(){
            end += 1;
            line.push(c);
            if let Ok(line_width) = glyphs.width(font_size, line.as_str()){
                if line_width > width {
                    break;
                }
            } 
        }
        
        (end, &self.text[start..end])
    }

}