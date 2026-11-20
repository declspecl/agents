use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variable {
    LambdaRuntimeApi,
    LocalBindAddress,
}

impl Variable {
    pub const ALL: [Variable; 2] = [Variable::LambdaRuntimeApi, Variable::LocalBindAddress];

    pub const fn as_str(self) -> &'static str {
        match self {
            Variable::LambdaRuntimeApi => "AWS_LAMBDA_RUNTIME_API",
            Variable::LocalBindAddress => "LOCAL_BIND_ADDRESS",
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
