use std::cmp::max;

use crate::{
    diagnostic::Diagnostic,
    utils::{IOFile, Style},
};

pub struct Logger;

impl Logger {
    fn generate_gutter_spaces(start_line: usize, end_line: usize, text_len: usize) -> String {
        let max_line_num_len = Self::num_length(max(start_line + 1, end_line + 1));

        " ".repeat(max_line_num_len - text_len)
    }

    fn generate_gutter_line_num(start_line: usize, end_line: usize, line_num: usize) -> String {
        let spaces = Self::generate_gutter_spaces(start_line, end_line, Self::num_length(line_num));

        format!(
            "{}{spaces}{line_num} |{}",
            Style::BRIGHT_BLACK,
            Style::RESET,
        )
    }

    fn generate_gutter_empty(start_line: usize, end_line: usize) -> String {
        let spaces = Self::generate_gutter_spaces(start_line, end_line, 0);

        format!("{}{spaces} |{}", Style::BRIGHT_BLACK, Style::RESET,)
    }

    fn generate_src(file: &IOFile, span_start: usize, span_end: usize) -> String {
        let mut src: Vec<String> = Vec::new();

        let start_line = file.search_line_index(span_start).unwrap();
        let end_line = file.search_line_index(span_end).unwrap();

        let lines = match file.get_lines(start_line, end_line) {
            Err(e) => panic!("UTF8Error: {}", e),
            Ok(l) => l,
        };

        // format the lines
        for (idx, line) in lines.iter().enumerate() {
            let line_idx = start_line + idx;

            let num_gutter = Self::generate_gutter_line_num(start_line, end_line, line_idx + 1);

            let fmt_line = format!("{num_gutter} {line}");

            src.push(fmt_line);

            // now, we need to mark the offending bits
            let empty_gutter = Self::generate_gutter_empty(start_line, end_line);

            let mark = format!(
                "{}{}{}{}{}{}",
                Style::BOLD,
                Style::BRIGHT_RED,
                " ".repeat(file.line_starts[line_idx] - span_start),
                "^".repeat(span_end - span_start),
                Style::RESET,
                Style::RESET_BOLD
            );

            src.push(format!("{empty_gutter} {mark}"));
        }

        src.join("\n")
    }

    pub fn generate_log(file: &IOFile, diag: Diagnostic) -> String {
        let mut log: Vec<String> = Vec::new();

        log.push(diag.to_string());

        log.push(format!(
            "{}-->{} {}",
            Style::BLUE,
            Style::RESET,
            file.path.to_str().unwrap().to_string()
        ));

        let src = Self::generate_src(file, diag.span.start, diag.span.end);
        log.push(src);

        log.join("\n")
    }

    fn num_length(mut num: usize) -> usize {
        let mut len: usize = 1;

        while num >= 10 {
            len += 1;
            num /= 10;
        }

        len
    }
}
