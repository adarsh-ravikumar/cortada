use core::fmt;

use crate::{
    common::{IOFile, Span},
    diagnostic::{Diagnostic, DiagnosticSeverity, Label, diagnostic::LabelKind},
    utils::Style,
};

pub struct DiagnosticRenderer<'a> {
    file: &'a IOFile,
    line_num_len: usize,
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Warn => write!(f, "{}warning", Style::BRIGHT_YELLOW,),
            Self::Error => write!(f, "{}error", Style::BRIGHT_RED,),
        }
    }
}

struct DiagnosticSpan {
    line: String,
    line_num: usize,
    start_col: usize,
    end_col: usize,
}

impl<'a> DiagnosticRenderer<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            file,
            line_num_len: 0,
        }
    }

    fn num_len(num: usize) -> usize {
        let mut num = num;
        let mut len = 0;

        loop {
            num /= 10;
            len += 1;

            if num == 0 {
                break;
            }
        }

        len
    }

    fn build_msg(&self, diag: &Diagnostic) -> String {
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

    fn build_loc_info(&self, diag: &Diagnostic) -> String {
        let (line, col) = self.file.line_col_from_index(diag.location.start);

        let spaces = " ".repeat(self.line_num_len + 1);
        format!(
            "{spaces}{}{}-->{} {}:{}:{}\n{spaces}{}{} │\n",
            Style::BOLD,
            Style::BRIGHT_BLACK,
            Style::RESET,
            self.file.path.display(),
            line,
            col,
            Style::BOLD,
            Style::BRIGHT_BLACK,
        )
    }

    fn break_span(&self, span: Span) -> Vec<DiagnosticSpan> {
        let (start_line, start_col) = self.file.line_col_from_index(span.start);
        let (mut end_line, mut end_col) = self.file.line_col_from_index(span.end);

        if end_col == 1 && end_line - start_line == 1 {
            end_line = start_line;
            end_col = span.start - self.file.line_starts[start_line - 1] + 1;
        }

        if start_line == end_line {
            vec![DiagnosticSpan {
                line: self.file.line(start_line).replace("\n", " "),
                line_num: start_line,
                start_col,
                end_col,
            }]
        } else {
            let start_line_end = span.start - self.file.line_starts[start_line - 1] + 1;

            vec![
                DiagnosticSpan {
                    line: self.file.line(start_line).replace("\n", " "),
                    line_num: start_line,
                    start_col,
                    end_col: start_line_end,
                },
                DiagnosticSpan {
                    line: self.file.line(end_line).replace("\n", " "),
                    line_num: end_line,
                    start_col: 0,
                    end_col,
                },
            ]
        }
    }

    fn build_paranthesised_line(&self, label: &Label, span: &DiagnosticSpan) -> String {
        let spaces = self.line_num_len - Self::num_len(span.line_num);

        if label.paranthesise {
            let (prefix, suffix) = span.line.split_at(span.start_col - 1);
            let (to_wrap, suffix) = suffix.split_at(span.end_col - span.start_col);

            format!(
                "{}{}{} {} │{}{} {}{}{}({}{}{}{}{}){}{}{}\n",
                " ".repeat(spaces),
                Style::BOLD,
                Style::BRIGHT_BLACK,
                span.line_num,
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
            )
        } else {
            format!(
                "{}{}{} {} │{}{} {}\n",
                " ".repeat(spaces),
                Style::BOLD,
                Style::BRIGHT_BLACK,
                span.line_num,
                Style::RESET,
                Style::RESET_BOLD,
                span.line,
            )
        }
    }

    fn build_highlight(
        &self,
        label: &Label,
        highlight_color: &str,
        mark: &str,
        span: &DiagnosticSpan,
    ) -> String {
        let num_spaces = span.start_col.checked_sub(1).unwrap_or(1);
        let num_carets = usize::max(span.end_col - span.start_col, 1);

        let space = " ".repeat(if label.paranthesise {
            num_spaces + 1
        } else {
            num_spaces
        });

        let highlight = mark.repeat(num_carets);

        format!(
            "{}{}{}│ {space}{}{highlight} {}{}{}\n",
            " ".repeat(self.line_num_len + 2),
            Style::BOLD,
            Style::BRIGHT_BLACK,
            highlight_color,
            label.msg,
            Style::RESET,
            Style::RESET_BOLD,
        )
    }

    fn build_labeled_src(
        &self,
        label: &Label,
        highlight_color: &str,
        mark: &str,
    ) -> (usize, String) {
        let lines = self.break_span(label.span);

        if lines.len() == 1 {
            let span = lines.first().unwrap();
            return (
                span.line_num,
                format!(
                    "{}{}",
                    self.build_paranthesised_line(label, span),
                    self.build_highlight(label, highlight_color, mark, span)
                ),
            );
        }

        let mut res = String::new();

        let first_span = lines.first().unwrap();
        let last_span = lines.last().unwrap();

        res.push_str(&format!(
            "{}{}",
            self.build_paranthesised_line(label, first_span),
            self.build_highlight(label, highlight_color, mark, first_span)
        ));

        if last_span.line_num - first_span.line_num > 1 {
            res.push_str(&format!(
                "{}{}...{}{}\n",
                Style::BOLD,
                Style::BRIGHT_BLACK,
                Style::RESET,
                Style::RESET_BOLD,
            ))
        }

        res.push_str(&format!(
            "{}{}",
            self.build_paranthesised_line(label, last_span),
            self.build_highlight(label, highlight_color, mark, last_span)
        ));

        (last_span.line_num, res)
    }

    fn build_labels(&self, diag: &Diagnostic) -> String {
        let mut res = String::new();

        let mut prev_line: usize = 0;

        for label in &diag.labels {
            let (last_line, to_render) = match label.kind {
                LabelKind::Primary => self.build_labeled_src(label, diag.severity.color(), "^"),

                LabelKind::Secondary => self.build_labeled_src(label, Style::BRIGHT_CYAN, "-"),
            };

            if prev_line == 0 {
                prev_line = last_line;
                res.push_str(&to_render);
                continue;
            }

            if last_line - prev_line > 1 {
                res.push_str(&format!(
                    " {}{}{}···{}{}\n",
                    " ".repeat(self.line_num_len),
                    Style::BOLD,
                    Style::BRIGHT_BLACK,
                    Style::RESET,
                    Style::RESET_BOLD,
                ))
            }
            if last_line == prev_line {
                res.push('\n');
            }

            res.push_str(&to_render);

            prev_line = last_line;
        }

        res
    }

    fn build_notes(&self, diag: &Diagnostic) -> String {
        let mut res = String::new();

        for note in &diag.notes {
            res.push_str(&format!(
                "{} {}{}={}{} note:{} {}\n",
                " ".repeat(self.line_num_len),
                Style::BOLD,
                Style::BRIGHT_BLACK,
                Style::RESET,
                Style::BOLD,
                Style::RESET_BOLD,
                note,
            ));
        }

        res
    }

    fn max_line_num_len(&self, diag: &Diagnostic) -> usize {
        let mut max: usize = 0;

        for label in diag.labels.iter() {
            let (line, _) = self.file.line_col_from_index(label.span.end);
            if line > max {
                max = line;
            }
        }

        Self::num_len(max)
    }

    pub fn render(&mut self, diagnostics: &mut Vec<Diagnostic>) -> String {
        let mut res = String::new();

        for diag in diagnostics.iter_mut() {
            diag.labels.sort_by_key(|label| label.span.start);

            self.line_num_len = self.max_line_num_len(diag);

            res.push_str(&self.build_msg(diag));
            res.push_str(&self.build_loc_info(diag));
            res.push_str(&self.build_labels(diag));
            res.push_str(&self.build_notes(diag));

            res.push('\n');
        }

        res.push_str(&format!(
            "{}{}error:{} could not compile due to {} previous errors",
            Style::BOLD,
            Style::BRIGHT_RED,
            Style::RESET,
            diagnostics.len(),
        ));

        res
    }
}
