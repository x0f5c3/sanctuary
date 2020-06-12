use mdbook::MDBook;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    count: u64,
}

impl Member {
    fn add<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let book = MDBook::load(path).unwrap();
        // This is probably the worst thing I ever wrote and probably ever will
        let title = match book.config.book.title {
            Some(st) => st,
            None => book
                .root
                .parent
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        };
        let path = book.root;
        let count = book.book.iter().count() as u64;
        Member {
            name: String::from(title),
            path,
            count,
        }
    }
}
