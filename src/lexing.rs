use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn find_tok_index(
    filepath: &str,
    row: usize,
    token: &str,
    line: &str,
) -> Vec<(String, usize, usize, String)> {
    // "("./filepath", row, token, line_as_a_string)"
    // [("./filepath", row, col, "token")]
    let v: Vec<_> = line.match_indices(token).collect();
    v.into_iter()
        .map(|(col, tok)| (filepath.to_owned(), row, col, tok.to_owned()))
        .collect()
}

fn lex_line(file_row_line: (&str, usize, String)) -> Vec<(String, usize, usize, String)> {
    // "("./filepath", row, line_as_a_string)"
    // [("program.ren", 3, 4, "+"), ("program.ren", row, col, "34")]
    let filepath = file_row_line.0.to_owned();
    let row = file_row_line.1;
    let line = file_row_line.2;
    let mut lexed_line = Vec::new();
    let tokens: Vec<String> = line
        .split_whitespace()
        .map(|token| token.to_owned())
        .collect();
    let uniq_tok = tokens.iter().unique().collect::<Vec<_>>();

    for token in uniq_tok {
        let mut col = find_tok_index(&filepath, row, token, &line);
        lexed_line.append(&mut col);
    }
    lexed_line.sort_by_key(|k| k.2);
    lexed_line
}

pub fn tokenize_file(filepath: &str) -> Vec<(String, usize, usize, String)> {
    // take in a filepath
    // assume that the file is 100 20
    // [("filepath", 0, 0, "100"), ("filepath", 0, 4, "20")] convert that file into a tuple of filepath, tokens, row and column

    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let vec_of_lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    let enumerated_vec_of_lines: Vec<(usize, &std::string::String)> =
        vec_of_lines.iter().enumerate().collect();
    enumerated_vec_of_lines
        .iter()
        .flat_map(|(row, line)| lex_line((&filepath, row.to_owned(), line.to_string())))
        .collect()
}
