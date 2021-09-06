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

    pub fn escape_code(&mut self, order: &str, v: i32) -> &mut Decobuilder {
        self.row_text(format!("\u{001b}[{a}{b}", a = v, b = order).as_str())
    }

    fn color_deco(&mut self, order: &str, n: i32, val: &str) -> &mut Decobuilder {
        self.escape_code(order, n).row_text(val)
    }

    pub fn plain_text(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 39)
    }

    pub fn black(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 30, s)
    }

    pub fn red(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 31, s)
    }

    pub fn green(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 32, s)
    }

    pub fn yellow(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 33, s)
    }

    pub fn blue(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 34, s)
    }

    pub fn magenta(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 35, s)
    }

    pub fn cyan(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 36, s)
    }

    pub fn white(&mut self, s: &str) -> &mut Decobuilder {
        self.color_deco("m", 37, s)
    }

    // Background color

    pub fn plain_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 49)
    }

    pub fn black_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 40)
    }

    pub fn red_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 41)
    }

    pub fn green_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 42)
    }

    pub fn yellow_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 43)
    }

    pub fn blue_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 44)
    }

    pub fn magenta_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 45)
    }

    pub fn cyan_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 46)
    }

    pub fn white_bg(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 47)
    }

    // type

    pub fn reset(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 0)
    }

    pub fn bold(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 1)
    }

    pub fn gloomy(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 2)
    }

    pub fn italic(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 3)
    }

    pub fn underline(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 4)
    }

    pub fn blink(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 5)
    }

    pub fn rapid_blink(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 6)
    }

    pub fn invert(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 7)
    }

    pub fn conceal(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 8)
    }

    pub fn strike(&mut self) -> &mut Decobuilder {
        self.escape_code("m", 9)
    }

    // Move the cursor

    pub fn move_cursor_up(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("A", n)
    }

    pub fn move_cursor_down(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("B", n)
    }

    pub fn move_cursor_right(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("C", n)
    }

    pub fn move_cursor_left(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("D", n)
    }

    pub fn move_cursor_down_begining(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("E", n)
    }

    pub fn move_cursor_up_begining(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("F", n)
    }

    pub fn move_cursor_from_left_edge(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("G", n)
    }

    // remove

    pub fn remove_screen(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("J", n)
    }

    pub fn remove_row(&mut self, n: i32) -> &mut Decobuilder {
        self.escape_code("K", n)
    }
}

#[cfg(test)]
mod tests {
    use super::Decobuilder;

    #[test]
    fn black_string() {
        let mut foo = Decobuilder::new("");
        foo.black("black");
        assert_eq!("\u{001b}[30mblack", foo.body);
    }
    #[test]
    fn red_string() {
        let mut foo = Decobuilder::new("");
        foo.red("red");
        assert_eq!("\u{001b}[31mred", foo.body);
    }
    #[test]
    fn green_string() {
        let mut foo = Decobuilder::new("");
        foo.green("green");
        assert_eq!("\u{001b}[32mgreen", foo.body);
    }
    #[test]
    fn yellow_string() {
        let mut foo = Decobuilder::new("");
        foo.yellow("yellow");
        assert_eq!("\u{001b}[33myellow", foo.body);
    }
    #[test]
    fn blue_string() {
        let mut foo = Decobuilder::new("");
        foo.blue("blue");
        assert_eq!("\u{001b}[34mblue", foo.body);
    }
    #[test]
    fn magenta_string() {
        let mut foo = Decobuilder::new("");
        foo.magenta("magenta");
        assert_eq!("\u{001b}[35mmagenta", foo.body);
    }
    #[test]
    fn cyan_string() {
        let mut foo = Decobuilder::new("");
        foo.cyan("cyan");
        assert_eq!("\u{001b}[36mcyan", foo.body);
    }
    #[test]
    fn white_string() {
        let mut foo = Decobuilder::new("");
        foo.white("white");
        assert_eq!("\u{001b}[37mwhite", foo.body);
    }

    // Background color

    #[test]
    fn black_bg() {
        let mut foo = Decobuilder::new("");
        foo.black_bg();
        assert_eq!("\u{001b}[40m", foo.body);
    }

    #[test]
    fn red_bg() {
        let mut foo = Decobuilder::new("");
        foo.red_bg();
        assert_eq!("\u{001b}[41m", foo.body);
    }

    #[test]
    fn green_bg() {
        let mut foo = Decobuilder::new("");
        foo.green_bg();
        assert_eq!("\u{001b}[42m", foo.body);
    }

    #[test]
    fn yellow_bg() {
        let mut foo = Decobuilder::new("");
        foo.yellow_bg();
        assert_eq!("\u{001b}[43m", foo.body);
    }

    #[test]
    fn blue_bg() {
        let mut foo = Decobuilder::new("");
        foo.blue_bg();
        assert_eq!("\u{001b}[44m", foo.body);
    }

    #[test]
    fn magenta_bg() {
        let mut foo = Decobuilder::new("");
        foo.magenta_bg();
        assert_eq!("\u{001b}[45m", foo.body);
    }

    #[test]
    fn cyan_bg() {
        let mut foo = Decobuilder::new("");
        foo.cyan_bg();
        assert_eq!("\u{001b}[46m", foo.body);
    }

    #[test]
    fn white_bg() {
        let mut foo = Decobuilder::new("");
        foo.white_bg();
        assert_eq!("\u{001b}[47m", foo.body);
    }

    // type
    #[test]
    fn reset() {
        let mut foo = Decobuilder::new("");
        foo.reset();
        assert_eq!("\u{001b}[0m", foo.body);
    }

    #[test]
    fn bold() {
        let mut foo = Decobuilder::new("");
        foo.bold();
        assert_eq!("\u{001b}[1m", foo.body);
    }

    #[test]
    fn gloomy() {
        let mut foo = Decobuilder::new("");
        foo.gloomy();
        assert_eq!("\u{001b}[2m", foo.body);
    }

    #[test]
    fn italic() {
        let mut foo = Decobuilder::new("");
        foo.italic();
        assert_eq!("\u{001b}[3m", foo.body);
    }

    #[test]
    fn underline() {
        let mut foo = Decobuilder::new("");
        foo.underline();
        assert_eq!("\u{001b}[4m", foo.body);
    }

    #[test]
    fn blink() {
        let mut foo = Decobuilder::new("");
        foo.blink();
        assert_eq!("\u{001b}[5m", foo.body);
    }

    #[test]
    fn rapid_blink() {
        let mut foo = Decobuilder::new("");
        foo.rapid_blink();
        assert_eq!("\u{001b}[6m", foo.body);
    }

    #[test]
    fn invert() {
        let mut foo = Decobuilder::new("");
        foo.invert();
        assert_eq!("\u{001b}[7m", foo.body);
    }

    #[test]
    fn conceal() {
        let mut foo = Decobuilder::new("");
        foo.conceal();
        assert_eq!("\u{001b}[8m", foo.body);
    }

    #[test]
    fn strike() {
        let mut foo = Decobuilder::new("");
        foo.strike();
        assert_eq!("\u{001b}[9m", foo.body);
    }

    // cursor

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
