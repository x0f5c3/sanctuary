extern crate dialoguer;
extern crate mdbook;
extern crate termcolor;

use dialoguer::Select;
use termcolor::WriteColor;

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::book_handler::Handler;
use crate::file_handler::{ConfigManagement, FileHandler, FileManagement};
use mdbook::{book::Chapter, config::Config, MDBook};
use printer::{Print, Printer};
use reader::{Read, Reader};
use types::ConfigFile::{Author, Editor, Repo, Title};
use utils::get_if_available;

pub mod book_handler;
pub mod file_handler;
mod git;
pub mod printer;
pub mod reader;
pub mod types;
pub mod utils;

pub struct IdeaBook<W, R> {
    pub fh: FileHandler,
    pub printer: Printer<W>,
    pub reader: Reader<R>,
    pub map: HashMap<u32, Chapter>,
}

impl<W, R> IdeaBook<W, R>
where
    W: Write + WriteColor,
    R: BufRead,
{
    pub fn run(&mut self) {
        if self.is_config_missing() {
            if self.is_first_time_run() {
                // If config dir is missing - create it
                if !self.fh.config_dir_exists() {
                    self.fh.config_dir_create().unwrap();
                }

                self.printer.print_fts_banner();
                self.setup_repo_path().unwrap();
                self.setup_editor_path().unwrap();
                self.setup_author().unwrap();
                self.setup_title().unwrap();
                self.setup_book();
            }

            // If repo path is missing - ask for it
            if self.fh.config_read(Repo).is_err() {
                self.setup_repo_path().unwrap();
            }

            // If editor path is missing - ask for it
            if self.fh.config_read(Editor).is_err() {
                self.setup_editor_path().unwrap();
            }
            if self.fh.config_read(Author).is_err() {
                self.setup_author().unwrap();
            }
            if self.fh.config_read(Title).is_err() {
                self.setup_title().unwrap();
            }

            self.printer
                .print("First time setup complete. Happy ideation!");
        } else {
            self.mapthebook();
            self.input_idea();
        }
    }

    pub fn clear_repo(&self) {
        if self.fh.config_read(Repo).is_ok() {
            self.fh
                .file_rm(Repo)
                .expect("Could not remove repo config file");
        }
    }
    pub fn mapthebook(&mut self) {
        let book = self.open_book();
        self.map = book.index_chapters();
    }
    pub fn open_book(&self) -> MDBook {
        match self.fh.config_read(Repo) {
            Ok(repo_path) => return MDBook::load(repo_path).unwrap(),
            Err(e) => panic!("No path to repository found: {}", e),
        }
    }
    pub fn setup_book(&self) {
        match self.fh.config_read(Repo) {
            Ok(repo_path) => match self.fh.config_read(Author) {
                Ok(author) => match self.fh.config_read(Title) {
                    Ok(title) => {
                        MDBook::create_idea_book(
                            std::path::PathBuf::from(repo_path),
                            title,
                            author,
                        )
                        .unwrap();
                    }
                    Err(e) => panic!("{}", e),
                },
                Err(e) => panic!("{}", e),
            },
            Err(e) => panic!("{}", e),
        }
    }

    pub fn read_chapter(&mut self, book: MDBook) {
        self.printer.print_chapter_selection_header();
        let select_index = Select::new()
            .items(&book.get_chapter_names())
            .default(0)
            .interact()
            .unwrap();
        book.prettyprint_chapter(select_index as u32 + 1, &self.map);
    }

    fn setup_author(&mut self) -> io::Result<()> {
        let mut author = String::new();
        while author.is_empty() {
            self.printer.print_author_input_header();
            self.printer.flush().unwrap();
            author = self.reader.read();
        }
        self.fh.config_write(Author, author)
    }
    fn setup_title(&mut self) -> io::Result<()> {
        let mut title = String::new();
        while title.is_empty() {
            self.printer.print_title_input_header();
            self.printer.flush().unwrap();
            title = self.reader.read();
        }
        self.fh.config_write(Title, title)
    }

    pub fn clear_editor(&self) {
        if self.fh.config_read(Editor).is_ok() {
            self.fh
                .file_rm(Editor)
                .expect("Could not remove editor config file");
        }
    }
    pub fn open_idea_existing(&mut self) {
        let repopath = self.fh.config_read(Repo).unwrap();
        let book = self.open_book();
        self.printer.print_chapter_selection_header();
        let select_index = Select::new()
            .items(&book.get_chapter_names())
            .default(0)
            .interact()
            .unwrap();
        let editorpath = self.fh.config_read(Editor).unwrap();
        let idea_summary = "Test summary".to_string();
        let chapter_path = book.get_chapter_path(select_index as u32 + 1, &self.map);
        if let Ok(_) = self.open_editor(&editorpath, &chapter_path.to_str().unwrap()) {
            self.add_idea_chapter(&chapter_path.to_str().unwrap(), idea_summary.clone());
            book.add_chapter_to_summary(&repopath.clone(), &idea_summary);
            git::add_and_commit(
                &repopath,
                chapter_path.to_str().unwrap(),
                idea_summary.clone(),
            )
            .unwrap();
        } else {
            panic!("LOL");
        }
    }

    fn setup_repo_path(&mut self) -> io::Result<()> {
        let mut input_repo_path = String::new();

        while input_repo_path.is_empty() {
            self.printer
                .print_input_header("Absolute path to your idea repo");
            self.printer.flush().unwrap();
            input_repo_path = self.reader.read();
        }

        self.fh.config_write(Repo, input_repo_path)
    }

    fn setup_editor_path(&mut self) -> io::Result<()> {
        self.printer.print_editor_selection_header();

        let select_index = Select::new()
            .default(0)
            .items(&["vim", "nano", "Other (provide name, e.g. 'emacs')"])
            .interact()
            .unwrap();

        let chosen_editor = match select_index {
            0 => "vim".to_string(),
            1 => "nano".to_string(),
            2 => {
                self.printer.print_input_header("");
                self.printer.flush().unwrap();
                self.reader.read()
            }
            _ => panic!("You should not be able to get here"),
        };

        let editor_path = get_if_available(chosen_editor.as_str()).unwrap_or_else(|| {
            panic!("Could not find executable for {} - aborting", chosen_editor)
        });

        self.fh.config_write(Editor, editor_path)
    }

    fn is_first_time_run(&self) -> bool {
        self.fh.config_read(Repo).is_err()
            && self.fh.config_read(Editor).is_err()
            && self.fh.config_read(Author).is_err()
            && self.fh.config_read(Title).is_err()
    }

    fn is_config_missing(&self) -> bool {
        self.fh.config_read(Repo).is_err()
            || self.fh.config_read(Editor).is_err()
            || self.fh.config_read(Author).is_err()
            || self.fh.config_read(Title).is_err()
    }
    fn add_idea_chapter(&mut self, path: &str, name: String) {
        let mut book = self.open_book();
        book.add_chapter(path, &name, None);
    }
    pub fn build_book(&mut self) {
        let book = self.open_book();
        book.build().unwrap();
    }

    fn input_idea(&mut self) {
        self.printer.print_input_header(">> Idea summary");
        let idea_summary = self.reader.read();
        let book = self.open_book();
        let editor_path = self.fh.config_read(Editor).unwrap();
        let repo_path = self.fh.config_read(Repo).unwrap();
        let chapter_path = format!("{}/src/{}.md", repo_path, idea_summary);
        if let Ok(_) = self.open_editor(&editor_path, &chapter_path) {
            self.add_idea_chapter(&chapter_path, idea_summary.clone());
            book.add_chapter_to_summary(&repo_path.clone(), &idea_summary);
            git::add_and_commit(&repo_path, &chapter_path, idea_summary.clone()).unwrap();
        } else {
            panic!("LOL");
        }
    }

    fn open_editor(&self, bin_path: &str, file_path: &str) -> io::Result<()> {
        match Command::new(bin_path).arg(file_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Error: Unable to open file [{}] with editor binary at [{}]: {}",
                    file_path, bin_path, e
                );
                Err(e)
            }
        }
    }
}
