#![allow(clippy::unwrap_used, reason = "tests and their helpers may unwrap")]

use std::collections::HashMap;

use api::environment::{
    BaseUrl,
    Environment,
    EnvironmentError,
    Runtime,
    Variable,
};

const LAMBDA: (&str, &str) = ("AWS_LAMBDA_RUNTIME_API", "127.0.0.1:9001");
const BASE_URL: (&str, &str) = ("PUBLIC_BASE_URL", "https://agents.example.com");

fn read(pairs: &[(&str, &str)]) -> Result<Environment, EnvironmentError> {
    let values: HashMap<String, String> = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    Environment::read_from(|name| values.get(name).cloned())
}

fn base_url(text: &str) -> BaseUrl {
    text.parse().unwrap()
}

#[test]
fn defaults_to_a_local_server_on_loopback() {
    let environment = read(&[]).unwrap();

    assert_eq!(
        environment.runtime,
        Runtime::Local {
            bind_address: "127.0.0.1:3000".parse().unwrap()
        }
    );
    assert_eq!(environment.base_url, base_url("http://127.0.0.1:3000"));
}

#[test]
fn lambda_runtime_api_selects_lambda() {
    assert_eq!(read(&[LAMBDA, BASE_URL]).unwrap().runtime, Runtime::Lambda);
}

#[test]
fn lambda_requires_the_public_base_url() {
    let error = read(&[LAMBDA]).unwrap_err();

    assert_eq!(error.problems.len(), 1);
    assert_eq!(error.problems[0].variable, Variable::PublicBaseUrl);
    assert_eq!(error.problems[0].message, "is required");
}

#[test]
fn lambda_ignores_the_local_bind_address() {
    let environment = read(&[LAMBDA, BASE_URL, ("LOCAL_BIND_ADDRESS", "0.0.0.0:8080")]).unwrap();

    assert_eq!(environment.runtime, Runtime::Lambda);
}

#[test]
fn reads_the_local_bind_address_and_derives_the_base_url_from_it() {
    let environment = read(&[("LOCAL_BIND_ADDRESS", "0.0.0.0:8080")]).unwrap();

    assert_eq!(
        environment.runtime,
        Runtime::Local {
            bind_address: "0.0.0.0:8080".parse().unwrap()
        }
    );
    assert_eq!(environment.base_url, base_url("http://0.0.0.0:8080"));
}

#[test]
fn an_explicit_base_url_wins_locally() {
    assert_eq!(read(&[BASE_URL]).unwrap().base_url, base_url("https://agents.example.com"));
}

#[test]
fn blank_values_count_as_unset() {
    assert_eq!(read(&[("AWS_LAMBDA_RUNTIME_API", "  "), ("LOCAL_BIND_ADDRESS", ""), ("PUBLIC_BASE_URL", " ")]), read(&[]));
}

#[test]
fn rejects_an_invalid_bind_address_naming_the_variable() {
    let error = read(&[("LOCAL_BIND_ADDRESS", "localhost")]).unwrap_err();

    assert_eq!(error.problems.len(), 1);
    assert_eq!(error.problems[0].variable, Variable::LocalBindAddress);
    assert!(error.to_string().contains("LOCAL_BIND_ADDRESS"));
}

#[test]
fn reports_every_invalid_variable_together() {
    let error = read(&[("LOCAL_BIND_ADDRESS", "nope"), ("PUBLIC_BASE_URL", "ftp://x")]).unwrap_err();
    let variables: Vec<_> = error.problems.iter().map(|problem| problem.variable).collect();

    assert_eq!(variables, [Variable::LocalBindAddress, Variable::PublicBaseUrl]);
}

#[test]
fn every_variable_has_a_distinct_name() {
    let mut names: Vec<_> = Variable::ALL.iter().map(|variable| variable.as_str()).collect();
    names.sort_unstable();
    names.dedup();

    assert_eq!(names.len(), Variable::ALL.len());
}

mod base_url {
    use super::*;

    #[test]
    fn trims_trailing_slashes_and_builds_absolute_urls() {
        let url = base_url("https://agents.example.com//");

        assert_eq!(url.as_str(), "https://agents.example.com");
        assert_eq!(url.to_absolute("/llms.txt"), "https://agents.example.com/llms.txt");
    }

    #[test]
    fn accepts_a_port() {
        assert_eq!(base_url("http://localhost:3000").to_string(), "http://localhost:3000");
    }

    #[test]
    fn rejects_anything_that_is_not_a_plain_origin() {
        for text in ["agents.example.com", "ftp://agents.example.com", "https://", "https://a.com/path", "https://a.com?x=1", "https://a.com#top", "https://a b.com"] {
            assert!(text.parse::<BaseUrl>().is_err(), "{text:?} should be rejected");
        }
    }
}
