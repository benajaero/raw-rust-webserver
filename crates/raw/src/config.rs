// Credit: Ben Ajaero

use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub threads: usize,
    pub doc_root: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Config, String> {
        let bind_addr = env::var("RWS_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
        let threads = match env::var("RWS_THREADS") {
            Ok(value) => parse_threads(&value)?,
            Err(_) => 4,
        };
        let doc_root = env::var("RWS_DOC_ROOT").unwrap_or_else(|_| "public".to_string());

        Ok(Config {
            bind_addr,
            threads,
            doc_root: PathBuf::from(doc_root),
        })
    }
}

fn parse_threads(value: &str) -> Result<usize, String> {
    let parsed: usize = value
        .parse()
        .map_err(|_| "RWS_THREADS must be a positive integer".to_string())?;
    if parsed == 0 {
        return Err("RWS_THREADS must be greater than zero".to_string());
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::parse_threads;

    #[test]
    fn parse_threads_rejects_zero() {
        assert!(parse_threads("0").is_err());
    }
}
