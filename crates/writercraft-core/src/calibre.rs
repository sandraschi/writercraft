//! Calibre Library Connector for WriterCraft
//!
//! Auto-discovers local Calibre libraries (from %APPDATA%/calibre/gui.json),
//! executes zero-lock read-only SQLite queries against metadata.db,
//! and extracts book metadata, series, formats, and blurbs.

use rusqlite::{params, Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalibreBook {
    pub id: i64,
    pub title: String,
    pub authors: Vec<String>,
    pub tags: Vec<String>,
    pub series: Option<String>,
    pub series_index: Option<f64>,
    pub formats: Vec<String>,
    pub comments: Option<String>,
    pub rel_path: String,
    pub pubdate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalibreLibraryInfo {
    pub name: String,
    pub path: String,
    pub total_books: usize,
    pub exists: bool,
}

pub struct CalibreConnector;

impl CalibreConnector {
    /// Auto-discovers Calibre libraries registered in Calibre GUI settings or default paths
    pub fn discover_libraries() -> Vec<CalibreLibraryInfo> {
        let mut paths: Vec<PathBuf> = Vec::new();

        // 1. Check %APPDATA%\calibre\gui.json
        if let Ok(appdata) = std::env::var("APPDATA") {
            let gui_json_path = Path::new(&appdata).join("calibre").join("gui.json");
            if gui_json_path.exists() {
                if let Ok(content) = fs::read_to_string(&gui_json_path) {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(stats) = val.get("library_usage_stats").and_then(|s| s.as_object()) {
                            for (lib_path, _) in stats {
                                let pb = PathBuf::from(lib_path);
                                if !paths.contains(&pb) {
                                    paths.push(pb);
                                }
                            }
                        }
                    }
                }
            }

            // 2. Check %APPDATA%\calibre\global.py.json
            let global_json_path = Path::new(&appdata).join("calibre").join("global.py.json");
            if global_json_path.exists() {
                if let Ok(content) = fs::read_to_string(&global_json_path) {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(lib_path) = val.get("library_path").and_then(|s| s.as_str()) {
                            let pb = PathBuf::from(lib_path);
                            if !paths.contains(&pb) {
                                paths.push(pb);
                            }
                        }
                    }
                }
            }
        }

        // 3. Common fallback locations
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            let user_lib = Path::new(&userprofile).join("Calibre Library");
            if !paths.contains(&user_lib) {
                paths.push(user_lib);
            }
        }

        let mut libraries = Vec::new();
        for p in paths {
            let db_path = p.join("metadata.db");
            if db_path.exists() {
                let count = Self::count_books(&db_path).unwrap_or(0);
                let name = p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Calibre Library")
                    .to_string();

                libraries.push(CalibreLibraryInfo {
                    name,
                    path: p.to_string_lossy().to_string(),
                    total_books: count,
                    exists: true,
                });
            }
        }

        // Sort by total books descending
        libraries.sort_by(|a, b| b.total_books.cmp(&a.total_books));
        libraries
    }

    /// Counts total books in metadata.db safely in read-only mode
    pub fn count_books(db_path: &Path) -> Result<usize, String> {
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        ).map_err(|e| format!("Failed to open {}: {}", db_path.display(), e))?;

        let count: i64 = conn.query_row("SELECT count(*) FROM books", [], |row| row.get(0))
            .map_err(|e| format!("Count failed: {}", e))?;

        Ok(count as usize)
    }

    /// Searches books in a Calibre library by query string matching title, author, or tag
    pub fn search_books(library_dir: &Path, query: &str, limit: usize) -> Result<Vec<CalibreBook>, String> {
        let db_path = library_dir.join("metadata.db");
        if !db_path.exists() {
            return Err(format!("metadata.db not found in {}", library_dir.display()));
        }

        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        ).map_err(|e| format!("Failed to open {}: {}", db_path.display(), e))?;

        let query_pattern = format!("%{}%", query.trim());

        let sql = r#"
            SELECT
                b.id,
                b.title,
                b.path,
                b.series_index,
                CAST(b.pubdate AS TEXT) AS pubdate_str,
                (SELECT group_concat(a.name, '|')
                   FROM books_authors_link bal
                   JOIN authors a ON a.id = bal.author
                  WHERE bal.book = b.id) AS authors_pipe,
                (SELECT s.name
                   FROM books_series_link bsl
                   JOIN series s ON s.id = bsl.series
                  WHERE bsl.book = b.id LIMIT 1) AS series_name,
                (SELECT group_concat(t.name, '|')
                   FROM books_tags_link btl
                   JOIN tags t ON t.id = btl.tag
                  WHERE btl.book = b.id) AS tags_pipe,
                (SELECT group_concat(upper(d.format), '|')
                   FROM data d
                  WHERE d.book = b.id) AS formats_pipe,
                (SELECT c.text
                   FROM comments c
                  WHERE c.book = b.id LIMIT 1) AS comments_text
            FROM books b
            WHERE b.title LIKE ?1
               OR authors_pipe LIKE ?1
               OR tags_pipe LIKE ?1
               OR series_name LIKE ?1
            ORDER BY b.id DESC
            LIMIT ?2
        "#;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| format!("Failed to prepare query: {}", e))?;

        let rows = stmt.query_map(params![query_pattern, limit as i64], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let rel_path: String = row.get(2)?;
            let series_index: Option<f64> = row.get(3)?;
            let pubdate: Option<String> = row.get(4)?;
            let authors_pipe: Option<String> = row.get(5)?;
            let series_name: Option<String> = row.get(6)?;
            let tags_pipe: Option<String> = row.get(7)?;
            let formats_pipe: Option<String> = row.get(8)?;
            let comments_text: Option<String> = row.get(9)?;

            let authors = authors_pipe
                .unwrap_or_default()
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let tags = tags_pipe
                .unwrap_or_default()
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let formats = formats_pipe
                .unwrap_or_default()
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            // Strip simple HTML tags from comments if present
            let comments = comments_text.map(|c| {
                let mut clean = String::new();
                let mut in_tag = false;
                for ch in c.chars() {
                    if ch == '<' {
                        in_tag = true;
                    } else if ch == '>' {
                        in_tag = false;
                    } else if !in_tag {
                        clean.push(ch);
                    }
                }
                clean.trim().to_string()
            });

            Ok(CalibreBook {
                id,
                title,
                authors,
                tags,
                series: series_name,
                series_index,
                formats,
                comments,
                rel_path,
                pubdate,
            })
        }).map_err(|e| format!("Query execution failed: {}", e))?;

        let mut results = Vec::new();
        for r in rows {
            if let Ok(book) = r {
                results.push(book);
            }
        }

        Ok(results)
    }

    /// Fetches single book detail by ID
    pub fn get_book(library_dir: &Path, book_id: i64) -> Result<Option<CalibreBook>, String> {
        let db_path = library_dir.join("metadata.db");
        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        ).map_err(|e| format!("Failed to open {}: {}", db_path.display(), e))?;

        let sql = r#"
            SELECT
                b.id,
                b.title,
                b.path,
                b.series_index,
                CAST(b.pubdate AS TEXT) AS pubdate_str,
                (SELECT group_concat(a.name, '|')
                   FROM books_authors_link bal
                   JOIN authors a ON a.id = bal.author
                  WHERE bal.book = b.id) AS authors_pipe,
                (SELECT s.name
                   FROM books_series_link bsl
                   JOIN series s ON s.id = bsl.series
                  WHERE bsl.book = b.id LIMIT 1) AS series_name,
                (SELECT group_concat(t.name, '|')
                   FROM books_tags_link btl
                   JOIN tags t ON t.id = btl.tag
                  WHERE btl.book = b.id) AS tags_pipe,
                (SELECT group_concat(upper(d.format), '|')
                   FROM data d
                  WHERE d.book = b.id) AS formats_pipe,
                (SELECT c.text
                   FROM comments c
                  WHERE c.book = b.id LIMIT 1) AS comments_text
            FROM books b
            WHERE b.id = ?1
        "#;

        let res = conn.query_row(sql, params![book_id], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let rel_path: String = row.get(2)?;
            let series_index: Option<f64> = row.get(3)?;
            let pubdate: Option<String> = row.get(4)?;
            let authors_pipe: Option<String> = row.get(5)?;
            let series_name: Option<String> = row.get(6)?;
            let tags_pipe: Option<String> = row.get(7)?;
            let formats_pipe: Option<String> = row.get(8)?;
            let comments_text: Option<String> = row.get(9)?;

            let authors = authors_pipe
                .unwrap_or_default()
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let tags = tags_pipe
                .unwrap_or_default()
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let formats = formats_pipe
                .unwrap_or_default()
                .split('|')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let comments = comments_text.map(|c| {
                let mut clean = String::new();
                let mut in_tag = false;
                for ch in c.chars() {
                    if ch == '<' {
                        in_tag = true;
                    } else if ch == '>' {
                        in_tag = false;
                    } else if !in_tag {
                        clean.push(ch);
                    }
                }
                clean.trim().to_string()
            });

            Ok(CalibreBook {
                id,
                title,
                authors,
                tags,
                series: series_name,
                series_index,
                formats,
                comments,
                rel_path,
                pubdate,
            })
        });

        match res {
            Ok(b) => Ok(Some(b)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }
}
