use clap::{App, Arg};
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Or, Predicate};
fn main() {
    let matches = App::new("Wikipedia Scraper")
        .version("0.0.1")
        .author("Robert Nubla")
        .about("Scrapes Wikipedia for a list of articles")
        .arg(
            Arg::new("search")
                .short('s')
                .long("search")
                .takes_value(true)
                .help("Search for articles"),
        )
        .get_matches();

    if matches.is_present("search") {
        let url = format!(
            "https://en.wikipedia.org/wiki/{}",
            matches.value_of("search").unwrap()
        );
        articles(&url).unwrap();
    }
}
#[tokio::main]
async fn articles(url: &str) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(url).await?;

    let body_response = resp.text().await?;
    // println!("{}", body_response);
    let parsed_html = Html::parse_document(&body_response);

    // let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#); example of accessiing attributes
    let selector = Selector::parse(r#"div[class="mw-parser-output"]"#).unwrap();
    let paragraph_selector = Selector::parse("p").unwrap();

    let content = parsed_html.select(&selector).next().unwrap();
    for p_tag in content.select(&paragraph_selector) {
        println!(
            "{}",
            // https://www.reddit.com/r/rust/comments/2ycl15/very_newb_printing_strings_without_quotes/
            // instead of {:?} use {} to remove escape characters
            format!("{}", p_tag.text().collect::<Vec<_>>().join(" "))
        );
    }

    Ok(())
}