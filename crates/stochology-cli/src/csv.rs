//! Pure-Rust CSV Table & Row Repository (`CsvTable`, `CsvRow`) for zero-dependency parsing of `data/*.csv`.

use std::fs;
use std::path::Path;

/// A single comma-separated row view with typed column accessors.
#[derive(Debug, Clone)]
pub struct CsvRow<'a> {
    cols: Vec<&'a str>,
}

impl<'a> CsvRow<'a> {
    /// Parses a comma-separated line into trimmed column slices.
    pub fn parse(line: &'a str) -> Self {
        Self {
            cols: line.split(',').map(str::trim).collect(),
        }
    }

    /// Number of columns in the row.
    #[inline]
    pub fn len(&self) -> usize {
        self.cols.len()
    }

    /// True if the row has no columns.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cols.is_empty()
    }

    /// Returns the raw trimmed column string at `index` (with surrounding quotes stripped if present).
    #[inline]
    pub fn str_at(&self, index: usize) -> Option<&'a str> {
        self.cols.get(index).map(|s| s.trim_matches('"'))
    }

    /// Parses column `index` as `f64`.
    #[inline]
    pub fn f64_at(&self, index: usize) -> Option<f64> {
        self.cols.get(index)?.parse::<f64>().ok()
    }

    /// Parses column `index` as `u32`.
    #[inline]
    pub fn u32_at(&self, index: usize) -> Option<u32> {
        self.cols.get(index)?.parse::<u32>().ok()
    }
}

/// Zero-dependency CSV table reader over `data/*.csv`.
#[derive(Debug, Clone)]
pub struct CsvTable {
    content: String,
}

impl CsvTable {
    /// Reads a CSV file from `root_dir.join(rel_path)`.
    pub fn open(root_dir: &Path, rel_path: &str) -> Result<Self, String> {
        let full_path = root_dir.join(rel_path);
        let content = fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read '{}': {e}", full_path.display()))?;
        Ok(Self { content })
    }

    /// Wraps an already-loaded CSV string.
    pub fn from_string(content: String) -> Self {
        Self { content }
    }

    /// Iterates over all data rows (skipping the header line).
    pub fn rows(&self) -> impl Iterator<Item = CsvRow<'_>> {
        self.content
            .lines()
            .skip(1)
            .filter(|l| !l.trim().is_empty())
            .map(CsvRow::parse)
    }
}
