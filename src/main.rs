// use owo_colors::OwoColorize;
use colored::Colorize;
use std::{cmp::Reverse, env, time::Instant};
use tabled::{
    settings::{object::Columns, Format, Style},
    Table, Tabled,
};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Tabled)]
struct FileItem {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(skip)]
    size: u64,
    #[tabled(rename = "Size")]
    formatted_size: String,
}
impl FileItem {
    fn new(name: &str, size: u64) -> Self {
        FileItem {
            name: name.to_string(),
            size: size,
            formatted_size: formatted_size(size),
        }
    }
}

fn formatted_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;

    match size {
        size if size < KB => format!("{}B", size),
        size if size < MB => format!("{:.2}KB", size as f64 / KB as f64),
        size if size < GB => format!("{:.2}MB", size as f64 / MB as f64),
        size if size < TB => format!("{:.2}GB", size as f64 / GB as f64),
        size => format!("{:.2} TB", size as f64 / TB as f64),
    }
}

impl From<DirEntry> for FileItem {
    fn from(entry: DirEntry) -> Self {
        let name = entry
            .path()
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        let size = get_entry_size(&entry);
        FileItem::new(name, size)
    }
}

fn get_entry_size(entry: &DirEntry) -> u64 {
    if entry.file_type().is_file() {
        return entry.metadata().map_or(0, |e| e.len());
    }
    let entry_path = entry.path();

    WalkDir::new(entry_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map_or(0, |m| m.len()))
        .sum()
}

fn list_dir(path: &str) {
    let start = Instant::now();

    let mut items: Vec<_> = WalkDir::new(path)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .map(FileItem::from)
        .collect();
    items.sort_by(|a, b| Reverse(a.size).cmp(&Reverse(b.size)));
    let table = Table::new(&items)
        .with(Style::rounded())
        .modify(Columns::single(0), Format::content(|s| s.red().to_string()))
        .modify(
            Columns::single(1),
            Format::content(|s| s.blue().to_string()),
        )
        .to_string();

    println!("{}", table);

    println!("\t共耗时: {:.2?}", start.elapsed())
}

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap_or(r"./".to_string());

    list_dir(path.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_dir() {
        list_dir(r"D:\codes")
    }
}
