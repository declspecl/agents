//! The process environment, read once at startup into a typed [`Environment`].
//!
//! Every variable the API reads is a [`Variable`], so none is read ad hoc elsewhere. Parsing reports every invalid
//! variable together, and takes a lookup function instead of touching the process environment, so tests need no global
//! state.

mod base_url;
mod error;
mod reader;
mod variable;

use std::net::SocketAddr;

pub use base_url::BaseUrl;
pub use error::{
    EnvironmentError,
    Problem,
};
use reader::Reader;
pub use variable::Variable;

const DEFAULT_LOCAL_BIND_ADDRESS: SocketAddr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), 3000);

/// How the process is hosted, which decides how it serves requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Runtime {
    Lambda,
    Local {
        bind_address: SocketAddr,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub runtime: Runtime,
    pub base_url: BaseUrl,
}

impl Environment {
    /// Reads the real process environment.
    pub fn read() -> Result<Environment, EnvironmentError> {
        Environment::read_from(|name| std::env::var(name).ok())
    }

    /// Reads from any source of values. A blank value counts as unset.
    pub fn read_from(lookup: impl Fn(&str) -> Option<String>) -> Result<Environment, EnvironmentError> {
        let mut reader = Reader::new(lookup);

        let in_lambda = reader.read_optional::<String>(Variable::LambdaRuntimeApi).is_some();
        let bind_address = reader.read_optional::<SocketAddr>(Variable::LocalBindAddress);
        let base_url = if in_lambda {
            reader.read_required::<BaseUrl>(Variable::PublicBaseUrl)
        } else {
            reader.read_optional::<BaseUrl>(Variable::PublicBaseUrl)
        };

        reader.finish()?;

        let bind_address = bind_address.unwrap_or(DEFAULT_LOCAL_BIND_ADDRESS);
        let runtime = if in_lambda {
            Runtime::Lambda
        } else {
            Runtime::Local {
                bind_address,
            }
        };
        // Inside Lambda the base URL is required, so the fallback only ever applies locally: the server's own address,
        // so a link in a local response works when followed.
        let base_url = base_url.unwrap_or_else(|| BaseUrl::from_address(bind_address));

        Ok(Environment {
            runtime,
            base_url,
        })
    }
}
