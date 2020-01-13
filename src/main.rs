use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match fetch_text(&args[1]) {
        Ok(result) => println!("{}", result.chars().count()),
        Err(e) => handler(e),
    }
}

fn fetch_text(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url)?.text()
}

fn handler(e: reqwest::Error) {
    if e.is_http() {
        match e.url() {
            None => println!("No Url given"),
            Some(url) => println!("Problem making request to: {}", url),
        }
    }
}
