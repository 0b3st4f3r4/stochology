//! Structured CLI and dataset verification error types (`CliError`) implementing `std::error::Error` and `fmt::Display`.

use std::fmt;
use stochology_core::{CardParseError, ContractViolation};

/// Structured error returned by dataset loaders, manifest verifiers, and empirical card audits.
#[derive(Debug, Clone, PartialEq)]
pub enum CliError {
    Io {
        path: String,
        message: String,
    },
    Utf8 {
        path: String,
        message: String,
    },
    ParseCard {
        path: String,
        error: CardParseError,
    },
    ContractAudit {
        card_id: String,
        violations: Vec<ContractViolation>,
    },
    ManifestMismatch {
        dataset_path: String,
        actual_sha256: String,
        expected_sha256: Option<String>,
    },
    UnsupportedDataset(String),
    EmpiricalMismatch {
        card_id: String,
        detail: String,
    },
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => write!(f, "Failed to read '{path}': {message}"),
            Self::Utf8 { path, message } => {
                write!(f, "Dataset '{path}' is not valid UTF-8: {message}")
            }
            Self::ParseCard { path, error } => write!(f, "Failed to parse '{path}': {error}"),
            Self::ContractAudit {
                card_id,
                violations,
            } => write!(
                f,
                "Card '{card_id}' failed structural audit: {violations:?}"
            ),
            Self::ManifestMismatch {
                dataset_path,
                actual_sha256,
                expected_sha256: Some(manifest_sha),
            } => write!(
                f,
                "Dataset '{dataset_path}' SHA-256 ({actual_sha256}) diverges from its SHA-256 manifest ({manifest_sha})"
            ),
            Self::ManifestMismatch {
                dataset_path,
                actual_sha256: _,
                expected_sha256: None,
            } => write!(
                f,
                "Dataset '{dataset_path}' is missing from its SHA-256 manifest"
            ),
            Self::UnsupportedDataset(other) => write!(
                f,
                "Unsupported primary card dataset path '{other}' (expected atlantic_storms, bank_calls, nyc_flights_2013, or cps_observational, or a protocol_profile with dataset_columns)"
            ),
            Self::EmpiricalMismatch { card_id, detail } => {
                write!(f, "Card '{card_id}' {detail}")
            }
        }
    }
}

impl std::error::Error for CliError {}

impl From<CliError> for String {
    fn from(err: CliError) -> Self {
        err.to_string()
    }
}
