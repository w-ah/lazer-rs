#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::io::{self, Write};

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

    pub fn print_ln(&self, msg: &str) -> &Printer {
        return self.print(msg).print("\n");
    }
}