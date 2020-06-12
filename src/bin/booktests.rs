extern crate mdbook;

use idea_book::book_handler::Handler;
use mdbook::book::{parse_summary, Book, BookItem, Chapter, Summary, SummaryItem};
use mdbook::config::Config;
use mdbook::MDBook;

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
    println!("{:#?}", sumpath);
    println!("{:#?}", summary);
}
