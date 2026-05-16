use code_chunk::chunk;

#[test]
fn splits_on_function_boundaries() {
    let src = "fn a() {\n    1\n}\nfn b() {\n    2\n}\n";
    // Tiny cap so the two fns don't merge into one chunk.
    let chunks = chunk(src, 10);
    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].contains("fn a()"));
    assert!(chunks[1].contains("fn b()"));
}

#[test]
fn merges_small_blocks_under_cap() {
    let src = "fn a() {\n    1\n}\nfn b() {\n    2\n}\nfn c() {\n    3\n}\n";
    let chunks = chunk(src, 10_000); // huge cap; one chunk holds all
    assert_eq!(chunks.len(), 1);
    assert!(chunks[0].contains("fn a()"));
    assert!(chunks[0].contains("fn b()"));
    assert!(chunks[0].contains("fn c()"));
}

#[test]
fn emits_oversize_block_whole() {
    // A single function larger than max_chars is returned alone.
    let body = "    x\n".repeat(200);
    let src = format!("fn big() {{\n{body}}}\n");
    let chunks = chunk(&src, 50);
    assert_eq!(chunks.len(), 1);
    assert!(chunks[0].contains("fn big()"));
}

#[test]
fn nested_braces_dont_double_split() {
    let src = "fn a() {\n    if x {\n        1\n    }\n}\nfn b() {}\n";
    let chunks = chunk(src, 30); // small cap so fns become separate chunks
    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].contains("if x"));
}

#[test]
fn trailing_non_block_is_kept() {
    let src = "fn a() {\n    1\n}\n// trailing comment\n";
    let chunks = chunk(src, 1000);
    assert!(chunks.iter().any(|c| c.contains("trailing comment")));
}
