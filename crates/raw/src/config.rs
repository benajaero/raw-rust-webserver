// Credit: Ben Ajaero

use std::env;

#[derive(Debug, Clone)]
pub struct RawConfig {
    pub bind_addr: String,
    pub worker_threads: usize,
}

impl RawConfig {
    pub fn from_env() -> Result<Self, String> {
        let bind_addr = env::var("RAW_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let worker_threads = match env::var("RAW_WORKERS") {
            Ok(value) => parse_workers(&value)?,
            Err(_) => 4,
        };

        Ok(Self {
            bind_addr,
            worker_threads,
        })
    }
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:3000".to_string(),
            worker_threads: 4,
        }
    }
}

fn parse_workers(value: &str) -> Result<usize, String> {
    let parsed: usize = value
        .parse()
        .map_err(|_| "RAW_WORKERS must be a positive integer".to_string())?;
    if parsed == 0 {
        return Err("RAW_WORKERS must be greater than zero".to_string());
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::parse_workers;

    #[test]
    fn parse_workers_rejects_zero() {
        assert!(parse_workers("0").is_err());
    }
}
