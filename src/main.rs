use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_vulpi() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_vulpi() }
}

fn main() {
    let mut parser = Parser::new();
    parser.set_language(language()).unwrap();

    println!("Hello, Vulpi!");
}
