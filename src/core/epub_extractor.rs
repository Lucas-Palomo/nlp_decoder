use epub::doc::EpubDoc;
use super::utils;

pub struct EpubExtractor {
    file_path: String
}

impl EpubExtractor {
    pub fn new(file_path: String) -> Self {
        EpubExtractor { file_path }
    }

    pub fn start(self) {
        let mut doc = EpubDoc::new(self.file_path)
            .expect("Cannot read the epub file");
//        let mut doc = read_doc.unwrap();
        doc.set_current_page(5).unwrap();

        let paragraphs = utils::find_tags(
            doc.get_current_str().unwrap(),
            "p".to_string());

        for paragraph in paragraphs {
            println!("{}", utils::parse_html(
                paragraph,
                "epub.localhost".to_string()));
        }
    }
}

