use std::error::Error;
use utoipa::OpenApi;

/// Small binary to export the OpenAPI (utoipa) specification to stdout.
///
/// Usage:
///   cargo run -p api-server --bin gen_openapi --release > openapi.json
///
/// This relies on the application exposing an `ApiDoc` OpenApi type in `crate::app` (see `src/app.rs`).
fn main() -> Result<(), Box<dyn Error>> {
    // Call the ApiDoc defined in the local crate's `app` module and print the pretty JSON.
    let openapi = api_server::app::ApiDoc::openapi();
    let json = openapi.to_pretty_json()?;
    println!("{json}");
    Ok(())
}
