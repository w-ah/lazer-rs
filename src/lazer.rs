use std::io::{self, Write};

const COLOR_RESET: &'static str = "\x1b[0m";
const COLOR_RED: &'static str = "\x1b[31m";

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

    pub fn print_color(&self, color: &str, msg: &str) -> &Printer {
        self.echo(color);
        self.print(msg);
        self.echo(COLOR_RESET);

        return self;
    }

    pub fn print_ln(&self, msg: &str) -> &Printer {
        return self.print(msg).print("\n");
    }

    pub fn print_color_ln(&self, color: &str, msg: &str) -> &Printer {
        self.print_color(color, msg);
        return self.print_ln("");
    }

    pub fn print_red(&self, msg: &str) -> &Printer {
        return self.print_color(COLOR_RED, msg);
    }

    pub fn reset(&self) -> &Printer {
        return self.print(COLOR_RESET);
    }
}

pub fn lazer() -> Printer {
    let printer = Printer::new();
    printer.reset();
    return printer;
}