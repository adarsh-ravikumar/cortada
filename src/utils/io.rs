use core::fmt;
use std::fs;
use std::path::PathBuf;

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

    pub fn search_line_index(&self, idx: usize) -> Option<usize> {
        let line = self.line_starts.partition_point(|&start| start <= idx);

        line.checked_sub(1)
    }

    pub fn get_line(&self, line: usize) -> Result<&str, std::str::Utf8Error> {
        let line_start = self.line_starts[line];

        let line_end = if line == self.line_starts.len() - 1 {
            self.src.len()
        } else {
            self.line_starts[line + 1] - 1
        };

        std::str::from_utf8(&self.src[line_start..line_end])
    }

    pub fn get_lines(&self, start: usize, end: usize) -> Result<Vec<&str>, std::str::Utf8Error> {
        let mut lines: Vec<&str> = Vec::new();

        for line in start..=end {
            lines.push(self.get_line(line)?)
        }

        Ok(lines)
    }

    pub fn view(&self, start: usize, end: usize) -> &str {
        std::str::from_utf8(&self.src[start..end]).unwrap()
    }
}
