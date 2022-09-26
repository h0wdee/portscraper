use portscraper::*;
use std::{env, process};

fn main() {
    // handle args
    let mut args = env::args().skip(1);
    let port = match args.next() {
        Some(n) => {
            // input check the lazy dooder's way, I'll clean this up l8r
            if n.parse::<usize>().expect("improper input") <= 65535 {
                n
            } else {
                eprintln!("usage: portscraper <port> | 0 >= port <= 65535");
                process::exit(1);
            }
        }
        None => {
            eprintln!("usage: portscraper <port>");
            process::exit(1);
        }
    };

    let target =
        String::from("https://web.archive.org/web/https://www.speedguide.net/port.php?port=");

    let response = do_gets(port.parse::<i32>().unwrap(), target).unwrap();

    if response.status().is_success() {
        let table = parse_table(&response.text().unwrap().as_str());
        // clean up the table for console output
        if let Some((w, _)) = term_size::dimensions() {
            println!(
                "{}",
                html2text::from_read(table.as_bytes(), w - (w / 8)) // clean up table output
            );
        } else {
            eprintln!("unable to get your terminal dimensions :(");
            process::exit(1);
        }
    } else {
        eprintln!("something happened: {:?}", response.status());
        process::exit(1);
    }
}
