use super::Decobuilder;

impl Decobuilder {
    pub fn new(s: String) -> Decobuilder {
        Decobuilder { body: s }
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
}

#[cfg(test)]
mod tests {
    use super::Decobuilder;

    #[test]
    fn black_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("black");
        assert_eq!("\u{001b}[31mblack\u{001b}[0m", foo.body);
    }
    #[test]
    fn red_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("red");
        assert_eq!("\u{001b}[31mred\u{001b}[0m", foo.body);
    }
    #[test]
    fn green_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("green");
        assert_eq!("\u{001b}[31mgreen\u{001b}[0m", foo.body);
    }
    #[test]
    fn yellow_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("yellow");
        assert_eq!("\u{001b}[31myellow\u{001b}[0m", foo.body);
    }
    #[test]
    fn blue_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("blue");
        assert_eq!("\u{001b}[31mblue\u{001b}[0m", foo.body);
    }
    #[test]
    fn magenta_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("magenta");
        assert_eq!("\u{001b}[31mmagenta\u{001b}[0m", foo.body);
    }
    #[test]
    fn cyan_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("cyan");
        assert_eq!("\u{001b}[31mcyan\u{001b}[0m", foo.body);
    }
    #[test]
    fn white_string() {
        let mut foo = Decobuilder::new("".to_string());
        foo.red("white");
        assert_eq!("\u{001b}[31mwhite\u{001b}[0m", foo.body);
    }
}
