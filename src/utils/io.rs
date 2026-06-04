use std::fs;
use std::path::PathBuf;

use crate::common::Span;

pub struct IOFile {
    pub path: PathBuf,
    pub src: Vec<u8>,
    pub line_starts: Vec<usize>,
}

impl IOFile {
    pub fn from_path(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path = path.into();

        let src = fs::read_to_string(&path)
            .map_err(|why| format!("Failed to open file {}: {}", path.display(), why))?;

        let src = src.into_bytes();
        let line_starts = Self::compute_line_starts(&src);

        Ok(IOFile {
            path,
            src,
            line_starts,
        })
    }

    pub fn get(&self, idx: usize) -> Option<&u8> {
        self.src.get(idx)
    }

    fn compute_line_starts(src: &Vec<u8>) -> Vec<usize> {
        let mut starts: Vec<usize> = vec![0];

        for (idx, byte) in src.iter().enumerate() {
            if *byte == b'\n' {
                starts.push(idx + 1);
            }
        }

        starts
    }

    pub fn line_from_index(&self, idx: usize) -> Option<usize> {
        let line = self.line_starts.partition_point(|&start| start <= idx);

        line.checked_sub(1)
    }

    pub fn line(&self, line: usize) -> &str {
        let line_start = self.line_starts[line - 1];

        let line_end = if line == self.line_starts.len() - 1 {
            self.src.len()
        } else {
            self.line_starts[line] - 1
        };

        std::str::from_utf8(&self.src[line_start..line_end])
            .expect("UTF-8 error when attempting to read source")
    }

    pub fn line_col_from_index(&self, index: usize) -> (usize, usize) {
        let line = self
            .line_from_index(index)
            .or_else(|| panic!("line index out of bounds!"))
            .unwrap();

        let line_start = self.line_starts[line];

        let col = index - line_start;

        (line + 1, col + 1)
    }

    pub fn index_from_line_col(&self, line: usize, col: usize) -> usize {
        self.line_starts[line] + col
    }

    pub fn view(&self, start: usize, end: usize) -> &str {
        std::str::from_utf8(&self.src[start..end]).unwrap()
    }

    pub fn view_span(&self, span: Span) -> &str {
        self.view(span.start, span.end)
    }
}
