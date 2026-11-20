mod error;
mod reader;
mod variable;

use std::net::SocketAddr;

pub use error::{
    EnvironmentError,
    Problem,
};
use reader::Reader;
pub use variable::Variable;

const DEFAULT_LOCAL_BIND_ADDRESS: SocketAddr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), 3000);

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
}

impl Environment {
    pub fn read() -> Result<Environment, EnvironmentError> {
        Environment::read_from(|name| std::env::var(name).ok())
    }

    pub fn read_from(lookup: impl Fn(&str) -> Option<String>) -> Result<Environment, EnvironmentError> {
        let mut reader = Reader::new(lookup);

        let lambda_runtime_api = reader.read_optional::<String>(Variable::LambdaRuntimeApi);
        let bind_address = reader.read_optional::<SocketAddr>(Variable::LocalBindAddress);

        reader.finish()?;

        let runtime = if lambda_runtime_api.is_some() {
            Runtime::Lambda
        } else {
            Runtime::Local {
                bind_address: bind_address.unwrap_or(DEFAULT_LOCAL_BIND_ADDRESS),
            }
        };

        Ok(Environment {
            runtime,
        })
    }
}
