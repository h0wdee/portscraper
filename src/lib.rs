use scraper::{Html, Selector};

// build the target out of an i32 and a String
pub fn build_target(x: i32, t: String) -> String {
    let s = &x.to_string()[..];
    t.clone() + s
}

// send GET request
pub fn do_gets(x: i32, t: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let backup_target =
        String::from("https://web.archive.org/save/www.speedguide.net/port.php?port=");
    let t = build_target(x, t);
    let resp = reqwest::blocking::get(t);
    if !resp.as_ref().unwrap().status().is_success() {
        eprintln!("error: {:?}", &resp.unwrap().status());
        eprintln!("attempting to archive...");
        let backup_t = build_target(x, backup_target);
        let backup_resp = reqwest::blocking::get(backup_t);
        return backup_resp;
    }
    resp
}

// grab the table element out of the html
pub fn parse_table(resp: &str) -> String {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("table.port").unwrap();
    let table = html.clone().select(&selector).fuse().next().unwrap().html();
    table
}
