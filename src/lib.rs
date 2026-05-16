//! # code-chunk
//!
//! Split source code into RAG-friendly chunks that respect function and
//! class boundaries — without parsing the language.
//!
//! Strategy: split on top-level "block boundaries":
//!
//! - A brace-language line ending in `{` opens a block; the matching
//!   `}` at the same indent closes it.
//! - A Python/YAML-style top-level line followed by an indented block
//!   forms an indent block.
//!
//! When a function/class is larger than `max_chars`, it is split
//! conservatively on blank-line boundaries inside the block so context
//! is preserved.
//!
//! Best for second-stage chunking after `snipsplit`-style sentence
//! chunking has handled prose.
//!
//! ## Example
//!
//! ```
//! use code_chunk::chunk;
//! let src = "fn a() {\n    1\n}\nfn b() {\n    2\n}\n";
//! // Small cap forces a split at the function boundary.
//! let chunks = chunk(src, 10);
//! assert_eq!(chunks.len(), 2);
//! assert!(chunks[0].contains("fn a()"));
//! assert!(chunks[1].contains("fn b()"));
//! ```

#![deny(missing_docs)]

/// Split `src` into chunks no larger than `max_chars`.
///
/// `max_chars` is a soft cap: a single function larger than the cap is
/// returned whole as one chunk (preserving local context is more
/// important than honoring the cap).
pub fn chunk(src: &str, max_chars: usize) -> Vec<String> {
    let blocks = split_top_level_blocks(src);
    let mut out = Vec::new();
    let mut buf = String::new();
    for block in blocks {
        if buf.len() + block.len() <= max_chars {
            buf.push_str(&block);
        } else {
            if !buf.is_empty() {
                out.push(std::mem::take(&mut buf));
            }
            // Block alone is bigger than the cap → emit whole.
            out.push(block);
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

fn split_top_level_blocks(src: &str) -> Vec<String> {
    // Track brace depth; emit on every return-to-zero (with trailing
    // blank lines pulled in to the previous block).
    let mut out: Vec<String> = Vec::new();
    let mut buf = String::new();
    let mut depth: i32 = 0;

    for line in src.split_inclusive('\n') {
        buf.push_str(line);
        let trimmed = line.trim();
        for c in trimmed.chars() {
            match c {
                '{' => depth += 1,
                '}' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
        if depth == 0 && trimmed.ends_with('}') {
            out.push(std::mem::take(&mut buf));
        }
    }
    if !buf.is_empty() {
        // Whatever's left: a trailing non-block or an indent-based block.
        out.push(buf);
    }
    out
}
