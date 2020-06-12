pub enum CliFlag {
    ClearRepo,
    ClearEditor,
    View,
    ShortView,
    BuildBook,
}

pub enum ConfigFile {
    Repo,
    Editor,
    Author,
    Title,
}

impl CliFlag {
    pub fn value(&self) -> &str {
        match *self {
            CliFlag::ClearRepo => "clear-repo",
            CliFlag::ClearEditor => "clear-editor",
            CliFlag::View => "view",
            CliFlag::ShortView => "v",
            CliFlag::BuildBook => "build-book",
        }
    }
}

impl ConfigFile {
    pub fn value(&self) -> &str {
        match *self {
            // These represents files so underscore is preferred
            ConfigFile::Repo => "repo_path",
            ConfigFile::Editor => "editor_path",
            ConfigFile::Author => "author_name",
            ConfigFile::Title => "book_title",
        }
    }
}
