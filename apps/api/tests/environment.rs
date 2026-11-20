use std::collections::HashMap;

use api::environment::{
    Environment,
    EnvironmentError,
    Runtime,
    Variable,
};

fn read(pairs: &[(&str, &str)]) -> Result<Environment, EnvironmentError> {
    let values: HashMap<String, String> = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
    Environment::read_from(|name| values.get(name).cloned())
}

#[test]
fn defaults_to_a_local_server_on_loopback() {
    assert_eq!(
        read(&[]).unwrap().runtime,
        Runtime::Local {
            bind_address: "127.0.0.1:3000".parse().unwrap()
        }
    );
}

#[test]
fn lambda_runtime_api_selects_lambda() {
    assert_eq!(read(&[("AWS_LAMBDA_RUNTIME_API", "127.0.0.1:9001")]).unwrap().runtime, Runtime::Lambda);
}

#[test]
fn lambda_ignores_the_local_bind_address() {
    let environment = read(&[("AWS_LAMBDA_RUNTIME_API", "127.0.0.1:9001"), ("LOCAL_BIND_ADDRESS", "0.0.0.0:8080")]).unwrap();

    assert_eq!(environment.runtime, Runtime::Lambda);
}

#[test]
fn reads_the_local_bind_address() {
    assert_eq!(
        read(&[("LOCAL_BIND_ADDRESS", "0.0.0.0:8080")]).unwrap().runtime,
        Runtime::Local {
            bind_address: "0.0.0.0:8080".parse().unwrap()
        }
    );
}

#[test]
fn blank_values_count_as_unset() {
    assert_eq!(read(&[("AWS_LAMBDA_RUNTIME_API", "  "), ("LOCAL_BIND_ADDRESS", "")]), read(&[]));
}

#[test]
fn rejects_an_invalid_bind_address_naming_the_variable() {
    let error = read(&[("LOCAL_BIND_ADDRESS", "localhost")]).unwrap_err();

    assert_eq!(error.problems.len(), 1);
    assert_eq!(error.problems[0].variable, Variable::LocalBindAddress);
    assert!(error.to_string().contains("LOCAL_BIND_ADDRESS"));
}

#[test]
fn every_variable_has_a_distinct_name() {
    let mut names: Vec<_> = Variable::ALL.iter().map(|variable| variable.as_str()).collect();
    names.sort_unstable();
    names.dedup();

    assert_eq!(names.len(), Variable::ALL.len());
}
