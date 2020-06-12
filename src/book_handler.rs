use crate::utils::get_if_available;
use mdbook::{
    book::{BookItem, Chapter, SectionNumber},
    config::Config,
    MDBook,
};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

pub trait Handler {
    fn add_chapter(&mut self, path: &str, name: &str, parents: Option<Vec<String>>);
    fn get_sum_path(&self) -> PathBuf;
    fn add_chapter_to_summary(&self, filename: &str, name: &str);
    fn get_chapter_paths(&self) -> Vec<PathBuf>;
    fn get_chapters(&self) -> Vec<&Chapter>;
    fn index_chapters(&self) -> HashMap<u32, Chapter>;
    fn get_chapter_path(&self, id: u32, map: &HashMap<u32, Chapter>) -> PathBuf;
    fn prettyprint_chapter(&self, id: u32, map: &HashMap<u32, Chapter>);
    fn get_chapter_nums(&self) -> Vec<SectionNumber>;
    fn get_parents(&self) -> Vec<Vec<String>>;
    fn get_chapter_names(&self) -> Vec<String>;
    fn create_idea_book(
        path: PathBuf,
        title: String,
        author: String,
    ) -> mdbook::errors::Result<MDBook>;
}
impl Handler for MDBook {
    fn add_chapter(&mut self, path: &str, name: &str, parents: Option<Vec<String>>) {
        let path_buf = PathBuf::from(path);
        let mut chapterfile = File::open(&path_buf).unwrap();
        let mut chaptercontent = String::new();
        let relativepath = path_buf.file_name().unwrap();
        chapterfile.read_to_string(&mut chaptercontent).unwrap();
        let chapter = match parents {
            Some(pars) => Chapter::new(name, chaptercontent, relativepath, pars),
            None => Chapter::new(name, chaptercontent, relativepath, Vec::new()),
        };
        self.book.push_item(chapter);
    }

    fn get_sum_path(&self) -> PathBuf {
        let mut srcpath = self.source_dir();
        srcpath.push("SUMMARY.md");
        return srcpath;
    }

    fn add_chapter_to_summary(&self, filename: &str, name: &str) {
        let sumpath = self.get_sum_path();
        let mut f = OpenOptions::new().append(true).open(sumpath).unwrap();
        writeln!(f, "- [{}](./{})", name, filename).unwrap();
    }

    fn get_chapter_paths(&self) -> Vec<PathBuf> {
        let mut out: Vec<PathBuf> = Vec::new();
        for item in self.book.iter() {
            match *item {
                BookItem::Chapter(ref chapter) => out.push(chapter.path.clone()),
                BookItem::Separator => {}
            }
        }
        out
    }

    fn get_chapters(&self) -> Vec<&Chapter> {
        let mut out: Vec<&Chapter> = Vec::new();
        for item in self.book.iter() {
            match *item {
                BookItem::Chapter(ref chapter) => out.push(chapter),
                BookItem::Separator => {}
            }
        }
        out
    }

    fn index_chapters(&self) -> HashMap<u32, Chapter> {
        let chaps = self.get_chapters();
        let mut out: HashMap<u32, Chapter> = HashMap::new();
        for chap in chaps {
            let num = chap.number.clone().unwrap()[0];
            out.insert(num, chap.to_owned());
        }
        out
    }

    fn get_chapter_path(&self, id: u32, map: &HashMap<u32, Chapter>) -> PathBuf {
        let chap = map.get(&id).unwrap();
        let mut bookpath = self.source_dir();
        bookpath.push(chap.path.clone());
        bookpath
    }

    fn prettyprint_chapter(&self, id: u32, map: &HashMap<u32, Chapter>) {
        let path = self.get_chapter_path(id, map);
        let bat_path =
            get_if_available("bat").expect("Cannot locate executable - less - on your system");
        match Command::new(bat_path).arg(&path).status() {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Error: Could not open idea file with bat at [{}]: {}",
                    path.to_str().unwrap(),
                    e
                );
            }
        }
    }

    fn get_chapter_nums(&self) -> Vec<SectionNumber> {
        let mut out: Vec<SectionNumber> = Vec::new();
        for item in self.book.iter() {
            match *item {
                BookItem::Chapter(ref chapter) => out.push(chapter.number.clone().unwrap()),
                BookItem::Separator => {}
            }
        }
        out
    }

    fn get_parents(&self) -> Vec<Vec<String>> {
        let mut out = Vec::new();

        for item in self.book.iter() {
            match *item {
                BookItem::Chapter(ref chapter) => out.push(chapter.parent_names.clone()),
                BookItem::Separator => {}
            }
        }
        out
    }

    fn get_chapter_names(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for item in self.book.iter() {
            match *item {
                BookItem::Chapter(ref chapter) => out.push(chapter.name.clone()),
                BookItem::Separator => {}
            }
        }
        out
    }

    fn create_idea_book(
        path: PathBuf,
        title: String,
        author: String,
    ) -> mdbook::errors::Result<MDBook> {
        let mut cfg = Config::default();
        cfg.book.title = Some(title);
        cfg.book.authors.push(author);
        MDBook::init(path)
            .create_gitignore(true)
            .with_config(cfg)
            .build()
    }
}
