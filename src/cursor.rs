
pub struct Cursor {
    pub position: CursorPosition
}

pub struct CursorPosition {
    pub line_num: u32,
    pub col_num: u32
}



impl Cursor{
    pub fn new() -> Cursor{
        Cursor{
            position: CursorPosition::new() 
        }
    }
}

impl CursorPosition{
    pub fn new() -> CursorPosition{
        CursorPosition{
            line_num: 0,
            col_num: 0
        }
    }
}