use std::io::{self, Write};
use chrono;

const COLOR_RESET: &'static str = "\x1b[0m";
const COLOR_RED: &'static str = "\x1b[31m";
const COLOR_GREEN: &'static str = "\x1b[32m";
const COLOR_YELLOW: &'static str = "\x1b[33m";
const COLOR_BLUE: &'static str = "\x1b[34m";
const COLOR_MAGENTA: &'static str = "\x1b[35m";
const COLOR_CYAN: &'static str = "\x1b[36m";

pub struct Printer {
    // Used to track state of calls to if/elseif/else()
    print_next: bool,
    block_entered: bool
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            print_next: true,
            block_entered: false
        }
    }

    fn echo(&mut self, msg: &str) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        let res = handle.write_all(msg.as_bytes());
        res.unwrap();
    }
    // if
    pub fn iff(&mut self, cond: bool) -> &mut Printer 
    {
        self.block_entered = cond;
        self.print_next = cond;
        return self;
    }
    // else if
    pub fn eliff(&mut self, cond: bool) -> &mut Printer {
        if self.block_entered {
            self.print_next = false;
            return self;
        }
        return self.iff(cond);
    }
    // else
    pub fn el(&mut self) -> &mut Printer {
        return self.eliff(true);
    }
    pub fn end(&mut self) -> &mut Printer {
        self.print_next = true;
        self.block_entered = false;
        return self;
    }
    pub fn print(&mut self, msg: &str) -> &mut Printer {
        if self.print_next {
            self.echo(msg);
        }
        return self;
    }
    fn print_color(&mut self, color: &str, msg: &str) -> &mut Printer {
        if !self.print_next {
            return self;
        }

        self.echo(color);
        self.print(msg);
        self.echo(COLOR_RESET);

        return self;
    }
    pub fn print_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print(msg).print("\n");
    }
    fn print_color_ln(&mut self, color: &str, msg: &str) -> &mut Printer {
        if !self.print_next {
            return self;
        }

        self.print_color(color, msg);
        return self.print_ln("");
    }
    pub fn print_red(&mut self, msg: &str) -> &mut Printer {
        return self.print_color(COLOR_RED, msg);
    }
    pub fn print_red_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print_color_ln(COLOR_RED, msg);
    }
    pub fn print_green(&mut self, msg: &str) -> &mut Printer {
        return self.print_color(COLOR_GREEN, msg);
    }
    pub fn print_green_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print_color_ln(COLOR_GREEN, msg);
    }
    pub fn print_yellow(&mut self, msg: &str) -> &mut Printer {
        return self.print_color(COLOR_YELLOW, msg);
    }
    pub fn print_yellow_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print_color_ln(COLOR_YELLOW, msg);
    }
    pub fn print_blue(&mut self, msg: &str) -> &mut Printer {
        return self.print_color(COLOR_BLUE, msg);
    }
    pub fn print_blue_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print_color_ln(COLOR_BLUE, msg);
    }
    pub fn print_magenta(&mut self, msg: &str) -> &mut Printer {
        return self.print_color(COLOR_MAGENTA, msg);
    }
    pub fn print_magenta_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print_color_ln(COLOR_MAGENTA, msg);
    }
    pub fn print_cyan(&mut self, msg: &str) -> &mut Printer {
        return self.print_color(COLOR_CYAN, msg);
    }
    pub fn print_cyan_ln(&mut self, msg: &str) -> &mut Printer {
        return self.print_color_ln(COLOR_CYAN, msg);
    }

    pub fn print_space(&mut self, len: usize) -> &mut Printer {
        let space_str = " ".repeat(len);
        return self.print(space_str.as_str());
    }
    pub fn print_utc_time(&mut self) -> &mut Printer {
        let now = chrono::offset::Utc::now();
        return self.print(&now.to_rfc2822());
    }
    pub fn print_pad_right(&mut self, msg: &str, len: usize, 
        delim: &str) -> &mut Printer {
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
    
    pub fn print_pad_left(&mut self, msg: &str, len: usize, 
        delim: &str) -> &mut Printer {
        let pad_len = len as i32 - msg.len() as i32;
        
        if pad_len > 0 {
            let mut msg_string = delim.repeat(pad_len as usize + 1);
            msg_string.push_str(msg);
            return self.print(&msg_string);
        }
        return self.print(msg);
    }

    fn set_color(&mut self, color: &str) -> &mut Printer {
        if !self.print_next {
            return self;
        }

        self.echo(color);
        return self;
    }
    pub fn reset(&mut self) -> &mut Printer {
        return self.set_color(COLOR_RESET);
    }
    pub fn set_color_red(&mut self) -> &mut Printer {
        return self.set_color(COLOR_RED);
    }
    pub fn set_color_green(&mut self) -> &mut Printer {
        return self.set_color(COLOR_GREEN);
    }
    pub fn set_color_yellow(&mut self) -> &mut Printer {
        return self.set_color(COLOR_YELLOW);
    }
    pub fn set_color_blue(&mut self) -> &mut Printer {
        return self.set_color(COLOR_BLUE);
    }
    pub fn set_color_magenta(&mut self) -> &mut Printer {
        return self.set_color(COLOR_MAGENTA);
    }
    pub fn set_color_cyan(&mut self) -> &mut Printer {
        return self.set_color(COLOR_CYAN);
    }
}

pub fn lazer() -> Printer {
    let mut printer = Printer::new();
    printer.reset();
    return printer;
}