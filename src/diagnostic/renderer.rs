use core::fmt;

use crate::{
    common::IOFile,
    diagnostic::{Diagnostic, DiagnosticSeverity, Label},
    utils::Style,
};

pub struct DiagnosticRenderer;

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warn => write!(f, "{}warning", Style::BRIGHT_YELLOW,),
            Self::Error => write!(f, "{}error", Style::BRIGHT_RED,),
        }
    }
}

impl DiagnosticRenderer {
    fn build_msg(diag: &Diagnostic) -> String {
        format!(
            "{}{}[{}]{}{} {}\n",
            Style::BOLD,
            diag.severity,
            diag.class.code(),
            Style::RESET,
            Style::RESET_BOLD,
            diag.msg
        )
    }

    fn build_loc_info(diag: &Diagnostic, file: &IOFile) -> String {
        let (line, col) = file.line_col_from_index(diag.primary.span.start);

        format!(
            "{}-->{} {}:{}:{}\n\n",
            Style::BRIGHT_BLUE,
            Style::RESET,
            file.path.display(),
            line,
            col
        )
    }

    fn build_labeled_src(label: &Label, file: &IOFile) -> String {
        let (start_line, start_col) = file.line_col_from_index(label.span.start);
        let (end_line, end_col) = file.line_col_from_index(label.span.end);

        if start_line != end_line {
            // this is a multi-line spanning highlight. we shall politely ignore
            return format!(
                "Mutli-line higlights not implemented. Unable to render source for message '{}'\n",
                label.msg
            );
        }

        let mut res = String::new();

        let line = file.line(start_line).replace("\n", "");

        if label.paranthesise {
            let (prefix, suffix) = line.split_at(start_col - 1);
            let (to_wrap, suffix) = suffix.split_at(end_col - start_col);
            res.push_str(&format!(
                "{}{} {} │{}{} {}{}{}({}{}{}{}{}){}{}{}\n",
                Style::BOLD,
                Style::BRIGHT_BLACK,
                start_line,
                Style::RESET,
                Style::RESET_BOLD,
                prefix,
                Style::BOLD,
                Style::BRIGHT_BLACK,
                Style::RESET,
                Style::RESET_BOLD,
                to_wrap,
                Style::BOLD,
                Style::BRIGHT_BLACK,
                Style::RESET,
                Style::RESET_BOLD,
                suffix
            ));
        } else {
            res.push_str(&format!(
                "{}{} {} │{}{} {}\n",
                Style::BOLD,
                Style::BRIGHT_BLACK,
                start_line,
                Style::RESET,
                Style::RESET_BOLD,
                line,
            ));
        }

        let line_num_len = Self::num_len(start_line);

        let num_spaces = label.span.start - file.line_starts[start_line - 1];
        let num_carets = usize::max(end_col - start_col, 1);

        let space = " ".repeat(num_spaces);
        let highlight = "^".repeat(num_carets);

        if label.paranthesise {
            res.push_str(&format!(
                "{}{}{}│ {space}{} {highlight} {}{}{}\n\n",
                " ".repeat(line_num_len),
                Style::BOLD,
                Style::BRIGHT_BLACK,
                Style::BRIGHT_RED,
                label.msg,
                Style::RESET,
                Style::RESET_BOLD,
            ));
        } else {
            res.push_str(&format!(
                "{}{}{} │ {space}{}{highlight} {}{}{}\n\n",
                " ".repeat(line_num_len),
                Style::BOLD,
                Style::BRIGHT_BLACK,
                Style::BRIGHT_RED,
                label.msg,
                Style::RESET,
                Style::RESET_BOLD,
            ));
        }

        res
    }

    fn build_notes(notes: &[String]) -> String {
        let mut res = String::new();

        for note in notes {
            res.push_str(&format!(
                "{}{}note: {}{}{}\n",
                Style::BOLD,
                Style::CYAN,
                note,
                Style::RESET,
                Style::RESET_BOLD
            ));
        }

        res
    }

    fn num_len(num: usize) -> usize {
        let mut num = num;
        let mut len = 1;

        loop {
            num /= 10;
            len += 1;

            if num == 0 {
                break;
            }
        }

        len
    }

    pub fn render(diag: Diagnostic, file: &IOFile) -> String {
        let mut res = String::new();

        res.push_str(&Self::build_msg(&diag));
        res.push_str(&Self::build_loc_info(&diag, file));
        res.push_str(&Self::build_labeled_src(&diag.primary, file));

        for sec in diag.secondary.iter() {
            res.push_str(&Self::build_labeled_src(sec, file));
        }

        res.push_str(&Self::build_notes(&diag.notes));

        res
    }
}
