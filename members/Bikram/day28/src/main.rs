// manin function is not allowed to be aync
/*
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
}
*/

use scraper::{Html, Selector};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let Some(url) = args.get(1) else {
        eprintln!("Usage: {} <url>", args.first().map(String::as_str).unwrap_or("program"));
        std::process::exit(1);
    };

    let runtime = tokio::runtime::Runtime::new().expect("failed to create Tokio runtime");

    runtime.block_on(async {
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });
}

async fn page_title(url: &str) -> Option<String> {
    let html = reqwest::get(url).await.ok()?.text().await.ok()?;

    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").ok()?;

    let title = document
        .select(&selector)
        .next()?
        .text()
        .collect::<String>();

    let title = title.trim();

    if title.is_empty() {
        None
    } else {
        Some(title.to_owned())
    }
}
