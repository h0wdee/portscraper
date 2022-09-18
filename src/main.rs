use scraper::{Html, Selector};
use std::{env, error, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
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

    let mut target = String::from(
        "https://web.archive.org/web/20210515122341/https://www.speedguide.net/port.php?port=",
    );
    target.push_str(port.as_str());

    let resp = reqwest::get(target).await?.text().await?; // this is the async part, can make better usage
    let html = Html::parse_document(resp.as_str());
    let selector = Selector::parse("table.port").unwrap();

    let table = html.select(&selector).fuse().next().unwrap();
    //  println!("{}", table.html()); // this is for scraping atm
    // this code is for uncached GET and table output to terminal
    if let Some((w, _)) = term_size::dimensions() {
        println!(
            "{}",
            html2text::from_read(table.html().as_bytes(), w - (w / 8)) // clean up table output
        );
    } else {
        eprintln!("Unable to get your terminal dimmensions :(");
        process::exit(1);
    }

    Ok(())
}
