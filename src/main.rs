use std::env;
use std::i64;

const CHAR_NAMES: [&'static str; 128] = [
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL",
    "BS", "TAB", "LF", "VT", "FF", "CR", "SO", "SI",
    "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB",
    "CAN", "EM", "SUB", "ESC", "FS", "GS", "RS", "US",
    " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")",
    "*", "+", ",", "-", ".", "/",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ":", ";", "<", "=", ">", "?",
    "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
    "V", "W", "X", "Y", "Z",
    "[", "\\", "]", "^", "_", "`",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
    "k", "l", "m", "n", "o", "p", "q", "r", "s", "t",
    "u", "v", "w", "x", "y", "z", "{", "|", "}", "~", "DEL",
];

fn print_table() {
    for i in 0..128 {
        print_row(i);
    }
}

fn print_row(i: i64) {
    println!("{:3}\t{:02X}\t{:08b}\t{:03o}\t{}", i, i, i, i, CHAR_NAMES[i as usize]);
}

fn parse_number(s: &str, radix: u32) -> Result<i64, &'static str> {
    let width = match radix {
        2 => 8,
        8 => 3,
        10 => 3,
        16 => 2,
        _ => 3
    };

    let digits = format!("{:0>width$}", s, width=width);
    let number = i64::from_str_radix(&digits, radix);
    match number {
        Ok(n) => {
            if n > 0x7f {
                Err("value out of range")
            }
            else {
                Ok(n.abs())
            }
        },
        Err(_) => Err("wrong format"),
    }
}

fn run_app(args: &[String]) -> Result<(), &'static str> {
    if args.len() == 0 {
        print_table();
        Ok(())
    }
    else {
        let arg = String::from(&args[0]);
        if arg.starts_with("me") {
            println!("Anything?");
            Ok(())
        }
        else {
            let number: Result<i64, &'static str>;
            if arg.starts_with("0x") {
                number = parse_number(&arg[2..], 16);
            }
            else if arg.starts_with("0b") {
                number = parse_number(&arg[2..], 2);
            }
            else if arg.starts_with("0o") {
                number = parse_number(&arg[2..], 8);
            }
            else {
                // assume the argument is a decimal number with no prefix
                number = parse_number(&arg, 10);
            }
            match number {
                Ok(n) => {
                    print_row(n);
                    Ok(())
                },
                Err(_) => Err("value out of range")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    std::process::exit(match run_app(&args[1..]) {
        Ok(_) => exitcode::OK,
        Err(err) => {
            eprintln!("error: {:?}", err);
            exitcode::USAGE
        }
    });
}
