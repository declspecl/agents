use std::fmt;
use std::str::FromStr;

/// The origin the site is served from: `http` or `https`, a host, and no path or trailing slash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseUrl {
    origin: String,
}

impl BaseUrl {
    /// The origin of a plain-HTTP server listening on `address`.
    pub fn from_address(address: std::net::SocketAddr) -> BaseUrl {
        BaseUrl {
            origin: format!("http://{address}"),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.origin
    }

    /// An absolute URL for a client path such as `/llms.txt`.
    pub fn to_absolute(
        &self,
        url_path: &str,
    ) -> String {
        format!("{}{url_path}", self.origin)
    }
}

impl FromStr for BaseUrl {
    type Err = String;

    fn from_str(text: &str) -> Result<BaseUrl, String> {
        let origin = text.trim_end_matches('/');
        let host = origin.strip_prefix("https://").or_else(|| origin.strip_prefix("http://")).ok_or("must start with http:// or https://")?;

        if host.is_empty() || host.contains(['/', '?', '#', ' ']) {
            return Err("must be an origin with a host and no path, query or fragment".to_owned());
        }

        Ok(BaseUrl {
            origin: origin.to_owned(),
        })
    }
}

impl fmt::Display for BaseUrl {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.origin)
    }
}
