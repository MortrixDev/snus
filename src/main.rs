use snus::{parse_markdown, Index};

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("md"))
        {
            out.push(path);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let query = env::args().collect::<Vec<_>>().join(" ");
    let dir = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: snus <dir>");
        process::exit(1);
    });

    let mut files = Vec::new();
    collect_files(Path::new(&dir), &mut files)?;

    let mut index = Index::default();

    for path in files {
        let content = fs::read_to_string(&path)?;
        index.add(parse_markdown(path, &content));
    }

    for (id, score) in index.search(&query).into_iter().take(10) {
        let doc = &index.docs[id as usize];
        println!(
            "{score:6.2}  {}  {}",
            doc.title.as_deref().unwrap_or("-"),
            doc.path.display()
        );
    }

    Ok(())
}
