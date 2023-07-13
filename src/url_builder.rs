use crate::{Metric, Result, root, CliError};


pub struct UrlBuilder<'a> {
    repo_name: &'a str,
    metric: Option<Metric>,
}

impl<'a> UrlBuilder<'a> {
    pub fn new(repo_name: &'a str) -> Self {
        Self {
            repo_name,
            metric: None,
        }
    }

    pub fn with_metric(self, metric: Metric) -> Self {
        Self {
            repo_name: self.repo_name,
            metric: Some(metric),
        }
    }

    pub fn build(self) -> Result<String> {
        match self.metric {
            Some(m) => {

                Ok(root.to_owned() + "")
            }
            None => Err(
                CliError::StringError(
                    format!("Should set metric before building the request url.")
                )
            )
        }
    }
}