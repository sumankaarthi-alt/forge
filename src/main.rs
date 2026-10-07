mod document;
mod index;
mod scanner;

use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "index" => {
            if args.len() < 3 {
                println!("Usage: forge index <directory>");
                return;
            }

            let path = std::path::Path::new(&args[2]);

            println!("Scanning project...");

            let start = Instant::now();

            let files = scanner::scan_directory(path);

            let mut search_index = index::SearchIndex::new();

            for file in &files {
                search_index.add_document(file);
            }

            let elapsed = start.elapsed();

            println!();
            println!("Index complete.");
            println!("Files indexed: {}", files.len());
            println!("Unique terms: {}", search_index.terms.len());
            println!("Index time: {:.2?}", elapsed);

            fs::create_dir_all(".forge").unwrap();

            search_index
                .save(".forge/index.json")
                .expect("Failed to save index");

            println!("Index saved: .forge/index.json");
        }

        "search" => {
            if args.len() < 3 {
                println!("Usage: forge search <query>");
                return;
            }

            let search_index = index::SearchIndex::load(".forge/index.json")
                .expect("No index found. Run `forge index <directory>` first.");

            let query = args[2..].join(" ");

            let start = Instant::now();

            let results = search_index.search(&query);

            let elapsed = start.elapsed();

            println!();
            println!("SEARCH: {}", query);
            println!("------------------------------");

            if results.is_empty() {
                println!("No matching documents found.");
            } else {
                for (rank, (document_id, score)) in results.iter().enumerate() {
                    if let Some(path) = search_index.documents.get(document_id) {
                        println!(
                            "{}. {}  [score: {}]",
                            rank + 1,
                            path,
                            score
                        );
                    }
                }
            }

            println!();
            println!(
                "{} result(s) in {:.2?}",
                results.len(),
                elapsed
            );
        }

        "stats" => {
            let search_index = index::SearchIndex::load(".forge/index.json")
                .expect("No index found. Run `forge index <directory>` first.");

            println!();
            println!("FORGE INDEX STATS");
            println!("------------------------------");
            println!("Documents:     {}", search_index.documents.len());
            println!("Unique terms:  {}", search_index.terms.len());

            if let Ok(metadata) = fs::metadata(".forge/index.json") {
                println!("Index size:     {} bytes", metadata.len());
            }
        }

        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Forge - Local Code Search Engine");
    println!();
    println!("Usage:");
    println!("  forge index <directory>");
    println!("  forge search <query>");
    println!("  forge stats");
}