//! Pure-Rust CSV Table & Row Repository (`CsvTable`, `CsvRow`) for zero-dependency parsing of `data/*.csv`.

use std::fs;
use std::path::Path;

/// A single comma-separated row view with typed column accessors.
#[derive(Debug, Clone)]
pub struct CsvRow<'a> {
    cols: Vec<&'a str>,
}

impl<'a> CsvRow<'a> {
    /// Parses a comma-separated line into trimmed column slices using a quote-aware RFC 4180 state machine
    /// so commas inside double-quoted fields (`"a, b"`) do not split the column.
    pub fn parse(line: &'a str) -> Self {
        let mut cols = Vec::new();
        let bytes = line.as_bytes();
        let mut start = 0usize;
        let mut in_quotes = false;
        let mut i = 0usize;
        while i < bytes.len() {
            match bytes[i] {
                b'"' => {
                    if in_quotes && i + 1 < bytes.len() && bytes[i + 1] == b'"' {
                        i += 2;
                        continue;
                    }
                    in_quotes = !in_quotes;
                }
                b',' if !in_quotes => {
                    cols.push(line[start..i].trim());
                    start = i + 1;
                }
                _ => {}
            }
            i += 1;
        }
        cols.push(line[start..].trim());
        Self { cols }
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

    /// Returns the parsed header row if the table is non-empty.
    pub fn header(&self) -> Option<CsvRow<'_>> {
        self.content
            .lines()
            .find(|l| !l.trim().is_empty())
            .map(CsvRow::parse)
    }

    /// Validates that the header and every non-empty data row have exactly `expected_cols` columns,
    /// returning the number of validated data rows.
    pub fn validate_schema(&self, expected_cols: usize) -> Result<usize, String> {
        let mut lines = self.content.lines().filter(|l| !l.trim().is_empty());
        let Some(header_line) = lines.next() else {
            return Err("CSV table is empty (missing header)".to_string());
        };
        let header_row = CsvRow::parse(header_line);
        if header_row.len() != expected_cols {
            return Err(format!(
                "Header column count mismatch: expected {expected_cols}, got {}",
                header_row.len()
            ));
        }
        let mut count = 0usize;
        for (idx, line) in lines.enumerate() {
            let row = CsvRow::parse(line);
            if row.len() != expected_cols {
                return Err(format!(
                    "Row {} column count mismatch: expected {expected_cols}, got {}",
                    idx + 1,
                    row.len()
                ));
            }
            count += 1;
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_table_and_row_accessors_work() {
        let table =
            CsvTable::from_string("col0,col1,col2\n\"alpha, beta\", 42, 3.5\n\n".to_string());
        assert_eq!(table.validate_schema(3), Ok(1));
        assert!(table.validate_schema(4).is_err());
        let bad_row = CsvTable::from_string("a,b\n1,2,3\n".to_string());
        assert!(bad_row.validate_schema(2).is_err());
        let rows: Vec<CsvRow<'_>> = table.rows().collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 3);
        assert!(!rows[0].is_empty());
        assert_eq!(rows[0].str_at(0), Some("alpha, beta"));
        assert_eq!(rows[0].u32_at(1), Some(42));
        assert_eq!(rows[0].f64_at(2), Some(3.5));
        assert_eq!(rows[0].str_at(9), None);
        assert!(CsvTable::open(Path::new("/nonexistent"), "missing.csv").is_err());
    }
}
