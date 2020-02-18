use webpage::{Webpage, WebpageOptions};
use super::utils;

pub struct WebExtractor {
    url: String
}

impl WebExtractor {
    pub fn new(url: String) -> Self {
        WebExtractor { url }
    }

    pub fn start(self) {
        let page = Webpage::from_url(
            self.url.as_ref(),
            WebpageOptions::default())
            .expect("Cannot connect to the webpage");

        let body = page.http.body
            .replace("<P", "<p")
            .replace("</P>", "</p>");
        let paragraphs = utils::find_tags(body, "p".to_string());

        for paragraph in paragraphs {
            println!("{}", utils::parse_html(paragraph.parse().unwrap(), (&self.url).parse().unwrap()));
        }
    }
}
