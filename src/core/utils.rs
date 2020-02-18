use regex::Regex;
use webpage::HTML;

// Create a Regex Pattern
pub fn tag_regex(tag: &str) -> Regex {
    return Regex::new(format!("<[{}][^>]*>(.+?)</[{}]>", tag, tag).as_str())
        .expect("Cannot create regex pattern");
}

// Return a Vec<String> of searched tags
pub fn find_tags(html: String, tag: String) -> Vec<String> {
    let regex = tag_regex(tag.as_str());
    return regex.find_iter(html.as_ref())
        .map(|mat| mat.as_str().to_string())
        .collect();
}

pub fn parse_html(html: String, url: String) -> String {
    let page = HTML::from_string(html, Option::from(url))
        .expect("Cannot read HTML fragment");
    return page.text_content;
}
