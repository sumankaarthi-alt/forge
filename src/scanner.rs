use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::document::Document;

pub fn scan_directory(path: &Path) -> Vec<Document> {
    let mut paths = Vec::new();

    collect_paths(path, &mut paths);

    paths
        .par_iter()
        .enumerate()
        .map(|(id, path)| {
            let content = fs::read_to_string(path).unwrap();

            Document {
                id,
                path: path.clone(),
                content,
            }
        })
        .collect()
}

fn collect_paths(path: &Path, files: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            if path.file_name().unwrap() == "target"
                || path.file_name().unwrap() == ".git"
            {
                continue;
            }

            collect_paths(&path, files);
        } else if is_source_file(&path) {
            files.push(path);
        }
    }
}

fn is_source_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("rs")
        | Some("c")
        | Some("cpp")
        | Some("h")
        | Some("java")
        | Some("py")
        | Some("js")
        | Some("ts")
        | Some("tsx")
        | Some("jsx") => true,

        _ => false,
    }
}