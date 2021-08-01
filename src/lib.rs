#![doc = include_str!("../README.md")]

use std::fmt;
use std::path::PathBuf;

use colored::Colorize;
use thiserror::Error;

pub use thisdiagnostic_derive::Diagnostic;

/**
Wrapper for errors that that includes a bit more additional metadata and includes additional details.
*/
#[derive(Error)]
#[error("{:?}", self)]
pub struct DiagnosticError {
    pub error: Box<dyn std::error::Error + Send + Sync>,
    pub label: String,
    pub help: Option<String>,
    pub meta: Option<DiagnosticMetadata>,
}

impl fmt::Debug for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(&self.error, f);
        } else {
            write!(f, "{}", self.label.red())?;
            match &self.meta {
                Some(DiagnosticMetadata::Net { ref url }) => {
                    write!(f, " @ {}", url.cyan().underline())?;
                }
                Some(DiagnosticMetadata::Fs { ref path }) => {
                    write!(f, " @ {}", path.to_string_lossy().cyan().underline())?;
                }
                Some(DiagnosticMetadata::Parse {
                    input: _input,
                    row,
                    col,
                    path,
                }) => {
                    write!(
                        f,
                        " - line: {}, col: {}",
                        row.to_string().green(),
                        col.to_string().green()
                    )?;
                    if let Some(path) = path {
                        write!(f, " @ {}", path.to_string_lossy().cyan().underline())?;
                    }
                }
                None => {}
            }
            write!(f, "\n\n")?;
            write!(f, "{:#}", self.error)?;
            if let Some(help) = &self.help {
                write!(f, "\n\n{}: {}", "help".yellow(), help)?;
            }
        }
        Ok(())
    }
}

pub type DiagnosticResult<T> = Result<T, DiagnosticError>;

impl<E> From<E> for DiagnosticError
where
    E: Diagnostic + Send + Sync,
{
    fn from(error: E) -> Self {
        Self {
            meta: error.meta(),
            label: error.label(),
            help: error.help(),
            error: Box::new(error),
        }
    }
}

/**
Optional additional metadata. This is used to improve diagnostic display, when present.
*/
#[derive(Debug)]
pub enum DiagnosticMetadata {
    Net {
        url: String,
    },
    Fs {
        path: PathBuf,
    },
    Parse {
        input: String,
        row: usize,
        col: usize,
        path: Option<PathBuf>,
    },
}

/**
Trait for manually adding label and help metadata to your error type.
*/
pub trait Diagnostic: std::error::Error + Send + Sync + 'static {
    fn label(&self) -> String;
    fn help(&self) -> Option<String>;
    fn meta(&self) -> Option<DiagnosticMetadata> {
        None
    }
}

// This is needed so Box<dyn Diagnostic> is correctly treated as an Error.
impl std::error::Error for Box<dyn Diagnostic> {}

pub trait IntoDiagnostic<T, E> {
    /**
    Helper for converting an error into a DiagnosticError with a custom label.

    Implemented for [std::result::Result] by default.

    ### Example
    ```ignore
    std::fs::read_file("./some_path").into_diagnostic("mytool::config::read_failure")?;
    ```
    */
    fn into_diagnostic(self, label: impl AsRef<str>) -> std::result::Result<T, DiagnosticError>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> IntoDiagnostic<T, E> for Result<T, E> {
    fn into_diagnostic(self, label: impl AsRef<str>) -> Result<T, DiagnosticError> {
        self.map_err(|e| DiagnosticError {
            error: Box::new(e),
            label: label.as_ref().into(),
            help: None,
            meta: None,
        })
    }
}
