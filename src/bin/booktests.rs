extern crate mdbook;

use mdbook::book::{parse_summary, Book, BookItem, Chapter, Summary, SummaryItem};
use mdbook::config::Config;
use mdbook::MDBook;
use sanctuary::book_handler::Handler;
use sanctuary::library::Library;

fn main() {
    let book = MDBook::load("/home/xc5/Projects/idea_book/testbook").unwrap();
    let root_dir = "/home/xc5/Projects/idea_book/create_test_book";
    let mut cfg = Config::default();
    cfg.book.title = Some("test".to_string());
    cfg.book.authors.push("test".to_string());
    let mut book2 = MDBook::init(root_dir)
        .create_gitignore(true)
        .with_config(cfg)
        .build()
        .unwrap();
    let summary = parse_summary("/home/xc5/Projects/idea_book/testbook/src/SUMMARY.md");
    let sumpath = book.get_sum_path();
    let j = serde_json::to_string(&book2.root).unwrap();
    println!("Book: {}", j);
}
