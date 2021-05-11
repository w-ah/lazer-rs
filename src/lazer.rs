use std::io::{self, Write};

const COLOR_RESET: &'static str = "\x1b[0m";
const COLOR_RED: &'static str = "\x1b[31m";
const COLOR_GREEN: &'static str = "\x1b[32m";
const COLOR_YELLOW: &'static str = "\x1b[33m";
const COLOR_BLUE: &'static str = "\x1b[34m";
const COLOR_MAGENTA: &'static str = "\x1b[35m";
const COLOR_CYAN: &'static str = "\x1b[36m";

pub struct Printer {

}

impl Printer {
    pub fn new() -> Printer {
        Printer {

        }
    }

    fn echo(&self, msg: &str) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        let res = handle.write_all(msg.as_bytes());
        res.unwrap();
    }
    pub fn print(&self, msg: &str) -> &Printer {
        self.echo(msg);
        return self;
    }
    fn print_color(&self, color: &str, msg: &str) -> &Printer {
        self.echo(color);
        self.print(msg);
        self.echo(COLOR_RESET);

        return self;
    }
    pub fn print_ln(&self, msg: &str) -> &Printer {
        return self.print(msg).print("\n");
    }
    fn print_color_ln(&self, color: &str, msg: &str) -> &Printer {
        self.print_color(color, msg);
        return self.print_ln("");
    }
    pub fn print_red(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_RED, msg);
    }
    pub fn print_red_ln(&self, msg: &str) -> &Printer {
        return self.print_color_ln(COLOR_RED, msg);
    }
    pub fn print_green(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_GREEN, msg);
    }
    pub fn print_green_ln(&self, msg: &str) -> &Printer {
        return self.print_color_ln(COLOR_GREEN, msg);
    }
    pub fn print_yellow(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_YELLOW, msg);
    }
    pub fn print_yellow_ln(&self, msg: &str) -> &Printer {
        return self.print_color_ln(COLOR_YELLOW, msg);
    }
    pub fn print_blue(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_BLUE, msg);
    }
    pub fn print_blue_ln(&self, msg: &str) -> &Printer {
        return self.print_color_ln(COLOR_BLUE, msg);
    }
    pub fn print_magenta(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_MAGENTA, msg);
    }
    pub fn print_magenta_ln(&self, msg: &str) -> &Printer {
        return self.print_color_ln(COLOR_MAGENTA, msg);
    }
    pub fn print_cyan(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_CYAN, msg);
    }
    pub fn print_cyan_ln(&self, msg: &str) -> &Printer {
        return self.print_color_ln(COLOR_CYAN, msg);
    }

    pub fn print_space(&self, len: usize) -> &Printer {
        let space_str = " ".repeat(len);
        return self.print(space_str.as_str());
    }
    pub fn print_utc_time(&self) -> &Printer {
        // TODO
        return self;
    }
    pub fn print_pad_right(&self, msg: &str, len: usize, 
        delim: &str) -> &Printer {
        let pad_len = len as i32 - msg.len() as i32;

        if pad_len > 0 {
            let mut msg_string = String::from(msg);
            msg_string.push_str(&delim.repeat(pad_len as usize + 1));

            return self.print(&msg_string);
        }
        else if pad_len < 0 {
            let pad_len_string = (-pad_len).to_string();
            let msg_string_slice = &msg[0..(len - pad_len_string.len())];
            
            let mut padded_msg = String::from(msg_string_slice);
            padded_msg.push_str("+");
            padded_msg.push_str(&pad_len_string);

            return self.print(&padded_msg);
        }
        return self.print(msg);
    }
    pub fn print_pad_left(&self) -> &Printer {
        // TODO
        return self;
    }

    
    fn set_color(&self, color: &str) -> &Printer {
        self.echo(color);
        return self;
    }
    pub fn reset(&self) -> &Printer {
        return self.set_color(COLOR_RESET);
    }
    pub fn set_color_red(&self) -> &Printer {
        return self.set_color(COLOR_RED);
    }
    pub fn set_color_green(&self) -> &Printer {
        return self.set_color(COLOR_GREEN);
    }
    pub fn set_color_yellow(&self) -> &Printer {
        return self.set_color(COLOR_YELLOW);
    }
    pub fn set_color_blue(&self) -> &Printer {
        return self.set_color(COLOR_BLUE);
    }
    pub fn set_color_magenta(&self) -> &Printer {
        return self.set_color(COLOR_MAGENTA);
    }
    pub fn set_color_cyan(&self) -> &Printer {
        return self.set_color(COLOR_CYAN);
    }
}

pub fn lazer() -> Printer {
    let printer = Printer::new();
    printer.reset();
    return printer;
}