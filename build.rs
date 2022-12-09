fn main() {
    cc::Build::new()
        .include("src")
        .file("src/parser.c")
        .compile("tree_sitter_vulpi");

    println!("cargo:rerun-if-changed=src/parser.c");
}
