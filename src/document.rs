pub struct Document {
    pub path: String,
    pub lines: Vec<String>,
}


impl Document {
    pub fn new() -> Document{
        Document{
            path: String::from("test.txt"),
            lines: vec![String::from("")]
        }
    }
}
