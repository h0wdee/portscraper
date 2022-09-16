use html2text;
use scraper::{Html, Selector};
use std::{env, error, process};
use term_size;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut args = env::args().skip(1);
    let port = match args.next() {
        Some(n) => n, // going to need to make sure this is a valid input at some point
        None => {
            eprintln!("usage: portscraper <port>");
            process::exit(1);
        }
    };

    let mut target = String::from("https://www.speedguide.net/port.php?port=");
    target.push_str(port.as_str());

    println!("Grabbing port {} info:", port);

    let resp = reqwest::get(target).await?.text().await?;
    let html = Html::parse_document(resp.as_str());
    let selector = Selector::parse("table.port").unwrap();

    let table = html.select(&selector).fuse().next().unwrap();
    if let Some((w, _)) = term_size::dimensions() {
        println!("wrapping at: {}", w);
        println!("{}", html2text::from_read(table.html().as_bytes(), w));
    } else {
        eprintln!("Unable to get your terminal dimmensions :(");
        process::exit(1);
    }

    Ok(())
}
