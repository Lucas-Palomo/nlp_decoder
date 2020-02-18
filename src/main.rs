use crate::core::decoder::Decoder;

mod core;

fn main() {
    const FILE_PATH: &'static str = "/home/darksrc/Desktop/epub/DOM_CASMURRO-MACHADO_DE_ASSIS.epub";
//    const URL: &'static str = "https://www.migalhas.com.br/depeso/196583/neoiluminismo";
    const URL: &'static str = "https://medium.com/@raphaevanges/um-metaleiro-no-big-brother-brasil-355fc412d1d3";

    let decoder = Decoder::new();
//    decoder.read_epub(FILE_PATH.parse().unwrap());
    decoder.read_webpage(URL.parse().unwrap());
}
