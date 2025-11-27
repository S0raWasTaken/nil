use std::fmt::Display;

use crate::Res;

/// Patterns:
/// installed:     [*] package-name-with-version description
/// not installed: [-] package-name-with-version description
#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub installed: bool,
}

use crate::colors::{BDIM, RED, RESET, YELLOW};

const PARSE_FAIL: &str = "\
[fatal] nil failed to parse xbps-query -Rs";

impl Package {
    pub fn new(package_str: &str) -> Res<Self> {
        let raw = package_str.trim().replacen('[', "", 1).replacen(']', "", 1);
        let mut raw = raw.split_whitespace();
        let installed = raw.next().map(|s| s == "*").ok_or(PARSE_FAIL)?;
        let name = raw.next().ok_or(PARSE_FAIL)?.to_string();
        let description = raw.collect::<Vec<_>>().join(" ");

        Ok(Self { name, description, installed })
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{RED}{}{RESET}] {YELLOW}{:<40}{RESET}{BDIM}{}{RESET}",
            if self.installed { "*" } else { "-" },
            self.name,
            clamp_str(&self.description, 70)
        )
    }
}

fn clamp_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len > 3 {
        format!("{}...", &s[..max_len - 3])
    } else {
        s[..max_len.min(s.len())].to_string()
    }
}
