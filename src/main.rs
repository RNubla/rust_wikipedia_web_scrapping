use clap::{App, Arg};
use scraper::{Html, Selector};

fn main() {
    let matches = App::new("Wikipedia Scraper")
        .version("0.1.0")
        .author("Robert Nubla")
        .about("Scrapes Wikipedia for a list of articles")
        .arg(
            Arg::new("search")
                .short('s')
                .long("search")
                .takes_value(true)
                .help("Search for articles"),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .takes_value(true)
                .help("Language to search in (default: en)\nFor a list of languages, see https://en.wikipedia.org/wiki/List_of_Wikipedias#Editions_overview"),
        )
        .get_matches();

    if matches.is_present("search") && matches.is_present("language") {
        let url = format!(
            "https://{}.wikipedia.org/wiki/{}",
            matches.value_of("language").unwrap(),
            matches.value_of("search").unwrap()
        );
        articles(&url).unwrap();
    } else if matches.is_present("search") {
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
