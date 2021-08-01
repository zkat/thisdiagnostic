# thisdiagnostic

`thisdiagnostic` is a Rust library for adding rich diagnostic metadata to
errors, for some really fancy and customizable error reporting!

## Example

### What You Write

```rust
use thisdiagnostic::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ApiError {
    /// Returned when a generic http client-related error has occurred.
    #[label("mytool::api::generic_http")]
    #[error("Request error:\n\t{0}")]
    HttpError(Box<dyn std::error::Error + Send + Sync>, String),

    /// Returned when a URL failed to parse.
    #[label("mytool::api::invalid_url")]
    #[help("Check the URL syntax. URLs must include the protocol part (https://, etc)")]
    #[error(transparent)]
    UrlParseError(#[from] Box<dyn std::error::Error + Send + Sync>),

    /// An API key is required.
    #[label("mytool::api::needs_api_key")]
    #[help("Please supply an API key.")]
    #[error("Endpoint operation requires an API key.")]
    NeedsApiKey,

    /// Unexpected response
    #[label("mytool::api::unexpected_response")]
    #[help("This is likely a bug with the server API (or its documentation). Please report it.")]
    #[error("Unexpected or undocumented response.")]
    BadResponse,
}
```

### What You Get

```ignore
$ ./mytool
Error: mytool::api::needs_api_key

Endpoint operation requires an API key.

help: Please supply an API key.
```

## License

This project and any contributions to it are [licensed under Apache 2.0](LICENSE.md).
