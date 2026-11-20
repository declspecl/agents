use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variable {
    LambdaRuntimeApi,
    LocalBindAddress,
    /// The public origin the site is served from, such as `https://agents.gavindhondt.com`, used for absolute links in
    /// `llms.txt` and `sitemap.xml`. Required inside Lambda. Locally it defaults to the local server's address.
    PublicBaseUrl,
}

impl Variable {
    pub const ALL: [Variable; 3] = [Variable::LambdaRuntimeApi, Variable::LocalBindAddress, Variable::PublicBaseUrl];

    pub const fn as_str(self) -> &'static str {
        match self {
            Variable::LambdaRuntimeApi => "AWS_LAMBDA_RUNTIME_API",
            Variable::LocalBindAddress => "LOCAL_BIND_ADDRESS",
            Variable::PublicBaseUrl => "PUBLIC_BASE_URL",
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
