use crate::core::epub_extractor::EpubExtractor;
use crate::core::web_extractor::WebExtractor;

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Decoder {}
    }

    pub fn read_epub(self, file_path: String) {
        let extractor = EpubExtractor::new(file_path);
        extractor.start();
    }

    pub fn read_webpage(self, url: String) {
        let extractor = WebExtractor::new(url);
        extractor.start();
    }
}
