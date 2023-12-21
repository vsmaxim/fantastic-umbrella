use crate::console::Console;

const BOX_LIGHT_BL_CORNER: &str = "└";
const BOX_LIGHT_BR_CORNER: &str = "┘";
const BOX_LIGHT_TR_CORNER: &str= "┐";
const BOX_LIGHT_TL_CORNER: &str = "┌";
const BOX_LIGHT_VERTICAL: &str = "│";
const BOX_LIGHT_HORIZONTAL: &str = "─";


pub struct Block {
    x: u16,
    y: u16,
    full_x: u16,
    full_y: u16,
    width: u16,
    height: u16,
    cursor_x: u16,
    cursor_y: u16,
    has_border: bool,
}


impl Block {
    pub fn new(
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        border: bool,
    ) -> Self {
        let (inner_x, inner_y, aw, ah) = if border {
            (x + 1, y + 1, width - 1, height - 1)
        } else {
            (x, y, width, height)
        };

        Self { 
            x: inner_x,
            y: inner_y,
            full_x: x,
            full_y: y,
            width: aw,
            height: ah,
            cursor_x: x,
            cursor_y: y,
            has_border: border,
        }
    }

    pub fn to_line_start(&mut self, console: &mut Console) {
        self.cursor_x = self.x;
        console.move_to(self.cursor_x, self.cursor_y);
    }

    pub fn next_line(&mut self, console: &mut Console) {
        self.cursor_y += 1;
        self.cursor_x = self.x;
        console.move_to(self.cursor_x, self.cursor_y);
    }

    pub fn reset(&mut self) {
        self.cursor_x = self.x;
        self.cursor_y = self.y;
    }

    pub fn draw_border(&mut self, console: &mut Console) {
        console.move_to(self.full_x, self.full_y);

        // Top box part
        console.write(BOX_LIGHT_TL_CORNER);
        console.write(BOX_LIGHT_HORIZONTAL.repeat(self.width.into()));
        console.write(BOX_LIGHT_TR_CORNER);

        // Vertical lines
        for cur_height in 1..self.height {
            console.move_to(self.full_x, self.full_y + cur_height);
            console.write(BOX_LIGHT_VERTICAL);
            console.move_to(self.full_x + self.width + 1, self.full_y + cur_height);
            console.write(BOX_LIGHT_VERTICAL);
        }

        // Bottom box part
        console.move_to(self.full_x, self.full_y + self.height);
        console.write(BOX_LIGHT_BL_CORNER);
        console.write(BOX_LIGHT_HORIZONTAL.repeat(self.width.into()));
        console.write(BOX_LIGHT_BR_CORNER);
    }

    pub fn render(&mut self, console: &mut Console) {
        if self.has_border {
            self.draw_border(console);
        }
    }

    pub fn write(&mut self, console: &mut Console, buf: &[u8]) {
        let mut in_escape_seq = false;
        let mut escape_seq = Vec::<u8>::new();

        for &b in buf {
            if self.cursor_y >= self.y + self.height {
                break; // Stop writing if height boundary is exceeded
            }

            if b == 0x0D { // Carriage Return
                self.to_line_start(console);
                continue;
            }

            // Handle new line and carriage return
            if b == 0x0A { // Line Feed
                self.next_line(console);
                continue;
            }

            // Escape sequence handling
            if b == 0x1B { // ESC character starts an escape sequence
                in_escape_seq = true;
                escape_seq.push(b);
                continue;
            }

            if in_escape_seq {
                escape_seq.push(b);

                if b == 0x48 { // Escape sequence is position control
                    let pos = String::from_utf8_lossy(&escape_seq[2..escape_seq.len() - 1]);

                    let coords: Vec<u16> = pos
                        .split(';')
                        .map(|v| v.parse::<u16>().unwrap())
                        .collect();


                    console.move_to(0, 100);
                    self.cursor_x = self.x + coords[1] - 1;
                    self.cursor_y = self.y + coords[0] - 1;

                    println!("{} {}", self.cursor_x, self.cursor_y);

                    in_escape_seq = false;
                    escape_seq.clear();
                    continue;
                }

                if (b >= 0x41 && b <= 0x5A) || (b >= 0x61 && b <= 0x7A) { // End of escape sequence
                    in_escape_seq = false;
                    console.write_raw(&escape_seq);
                    escape_seq.clear();
                    continue;
                }
            } else {
                // Regular character handling
                console.move_to(self.cursor_x, self.cursor_y);
                console.write_raw(&[b]);
                self.cursor_x += 1;

                if self.cursor_x >= self.x + self.width {
                    self.next_line(console);
                }
            }
        }

        console.flush();
    }

    pub fn write_str(&mut self, console: &mut Console, s: &str) {
        self.write(console, s.as_bytes());
    }
}

