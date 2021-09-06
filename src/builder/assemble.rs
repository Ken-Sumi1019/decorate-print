use super::Decobuilder;

impl Decobuilder {
    pub fn new(s: &str) -> Decobuilder {
        Decobuilder {
            body: s.to_string(),
        }
    }

    pub fn row_text(&mut self, s: &str) -> &mut Decobuilder {
        self.body.push_str(s);
        self
    }

    pub fn escape_code(&mut self, order: &str, v: &str) -> &mut Decobuilder {
        self.row_text(format!("\u{001b}[{a}{b}", a = v, b = order).as_str())
    }

    fn color_deco(&mut self, order: &str, n: &str, val: &str) -> &mut Decobuilder {
        self.escape_code(order, n)
            .row_text(val)
            .escape_code(order, "0")
    }

    pub fn black(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "30", s)
    }

    pub fn red(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "31", s)
    }

    pub fn green(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "32", s)
    }

    pub fn yellow(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "33", s)
    }

    pub fn blue(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "34", s)
    }

    pub fn magenta(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "35", s)
    }

    pub fn cyan(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "36", s)
    }

    pub fn white(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", "37", s)
    }

    // Move the cursor

    pub fn move_cursor_up(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("A", &n.to_string())
    }

    pub fn move_cursor_down(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("B", &n.to_string())
    }

    pub fn move_cursor_right(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("C", &n.to_string())
    }

    pub fn move_cursor_left(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("D", &n.to_string())
    }

    pub fn move_cursor_down_begining(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("E", &n.to_string())
    }

    pub fn move_cursor_up_begining(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("F", &n.to_string())
    }

    pub fn move_cursor_from_left_edge(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("G", &n.to_string())
    }

    // remove

    pub fn remove_screen(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("J", &n.to_string())
    }

    pub fn remove_row(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("K", &n.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Decobuilder;

    #[test]
    fn black_string() {
        let mut foo = Decobuilder::new("");
        foo.red("black");
        assert_eq!("\u{001b}[31mblack\u{001b}[0m", foo.body);
    }
    #[test]
    fn red_string() {
        let mut foo = Decobuilder::new("");
        foo.red("red");
        assert_eq!("\u{001b}[31mred\u{001b}[0m", foo.body);
    }
    #[test]
    fn green_string() {
        let mut foo = Decobuilder::new("");
        foo.red("green");
        assert_eq!("\u{001b}[31mgreen\u{001b}[0m", foo.body);
    }
    #[test]
    fn yellow_string() {
        let mut foo = Decobuilder::new("");
        foo.red("yellow");
        assert_eq!("\u{001b}[31myellow\u{001b}[0m", foo.body);
    }
    #[test]
    fn blue_string() {
        let mut foo = Decobuilder::new("");
        foo.red("blue");
        assert_eq!("\u{001b}[31mblue\u{001b}[0m", foo.body);
    }
    #[test]
    fn magenta_string() {
        let mut foo = Decobuilder::new("");
        foo.red("magenta");
        assert_eq!("\u{001b}[31mmagenta\u{001b}[0m", foo.body);
    }
    #[test]
    fn cyan_string() {
        let mut foo = Decobuilder::new("");
        foo.red("cyan");
        assert_eq!("\u{001b}[31mcyan\u{001b}[0m", foo.body);
    }
    #[test]
    fn white_string() {
        let mut foo = Decobuilder::new("");
        foo.red("white");
        assert_eq!("\u{001b}[31mwhite\u{001b}[0m", foo.body);
    }

    #[test]
    fn move_cursor_up() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_up(5);
        assert_eq!("sample text\u{001b}[5A", foo.body);
    }

    #[test]
    fn move_cursor_down() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_down(5);
        assert_eq!("sample text\u{001b}[5B", foo.body);
    }

    #[test]
    fn move_cursor_right() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_right(5);
        assert_eq!("sample text\u{001b}[5C", foo.body);
    }

    #[test]
    fn move_cursor_left() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_left(5);
        assert_eq!("sample text\u{001b}[5D", foo.body);
    }

    #[test]
    fn move_cursor_down_begining() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_down_begining(5);
        assert_eq!("sample text\u{001b}[5E", foo.body);
    }

    #[test]
    fn move_cursor_up_begining() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_up_begining(5);
        assert_eq!("sample text\u{001b}[5F", foo.body);
    }

    #[test]
    fn move_cursor_from_left_edge() {
        let mut foo = Decobuilder::new("sample text");
        foo.move_cursor_from_left_edge(5);
        assert_eq!("sample text\u{001b}[5G", foo.body);
    }

    #[test]
    fn remove_screen() {
        let mut foo = Decobuilder::new("sample text");
        foo.remove_screen(0);
        assert_eq!("sample text\u{001b}[0J", foo.body);
    }

    #[test]
    fn remove_row() {
        let mut foo = Decobuilder::new("sample text");
        foo.remove_row(0);
        assert_eq!("sample text\u{001b}[0K", foo.body);
    }
}
