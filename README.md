\# Forge 🦀



A lightweight local code-search and indexing engine built from scratch in Rust.



Forge recursively scans a codebase, indexes source files, persists the index, and provides ranked search results through a CLI.



\## Features



\- Recursive source-code scanning

\- Parallel file processing with Rayon

\- Inverted index using Rust HashMaps

\- Case-insensitive tokenization

\- Relevance scoring

\- Persistent JSON index

\- Search and indexing benchmarks

\- CLI interface

\- Index statistics



\## Usage



\### Index a project



```bash

cargo run -- index ./my-project

```



\### Search the index



```bash

cargo run -- search authentication

```



\### View statistics



```bash

cargo run -- stats

```



\## Example



```text

SEARCH: document

\------------------------------

1\. .\\src\\index.rs  \[score: 8]

2\. .\\src\\scanner.rs  \[score: 4]

3\. .\\src\\document.rs  \[score: 1]

4\. .\\src\\main.rs  \[score: 1]



4 result(s) in 67.40µs

```



\## Architecture



```text

Codebase

&#x20;  ↓

File Scanner

&#x20;  ↓

Parallel File Processing

&#x20;  ↓

Document Collection

&#x20;  ↓

Inverted Index

&#x20;  ↓

Persistent JSON Index

&#x20;  ↓

Query

&#x20;  ↓

Ranked Results

```



\## Tech Stack



\- Rust

\- Rayon

\- Serde

\- Serde JSON



\## Why I Built It



I wanted to learn Rust by building something larger than a tutorial project.



The goal was to understand Rust's approach to systems programming while implementing a practical search and indexing pipeline from scratch.



\## Future Improvements



\- Better code-aware tokenization

\- TF-IDF / BM25 ranking

\- Fuzzy search

\- Line-level results and code snippets

\- Incremental indexing

\- AI-assisted code retrieval

