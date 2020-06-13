use mdbook::book::Book;
use mdbook::book::BookItem;
use mdbook::MDBook;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    name: String,
    books: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    name: String,
    path: PathBuf,
    parts: Vec<Part>,
    count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    name: String,
    path: PathBuf,
}

impl Library {
    fn load(path: &str) -> Self
    {
        let mut contents = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut contents).unwrap();
        let res: Library = from_str(&contents).unwrap();
        res
    }
}

impl Member {
    fn add<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let book = MDBook::load(path).unwrap();
        let mut title = String::new();
        if let Some(st) = book.config.book.title {
            title = st;
        }
        // This is probably the worst thing I ever wrote and probably ever will
        // TODO Please fix this if I won't, there need to be checks for this
        // Not really fixed but you can at least see how bad it is
        title = book.root.parent().unwrap().file_name().unwrap().to_str().unwrap().to_owned();

        let path = book.root;
        let count = book.book.iter().count() as u64;
        let parts = Member::collect_parts(&path, &book.book);
        Member {
            name: title,
            path,
            parts,
            count,
        }
    }
    fn collect_parts(rootpath: &PathBuf, book: &Book) -> Vec<Part> {
        let mut res: Vec<Part> = Vec::new();
        let mut srcpath = rootpath.clone();
        srcpath.push("src/");
        for items in book.iter() {
            match items {
                BookItem::Separator => {}
                BookItem::Chapter(chap) => {
                    let mut fullpath = srcpath.clone();
                    fullpath.push(chap.path.clone());
                    let part = Part {
                        name: chap.name.clone(),
                        path: fullpath,
                    };
                    res.push(part);
                }
            }
        }
        res
    }
}
