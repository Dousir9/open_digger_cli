use crate::metric::Metric;

pub struct UrlBuilder<'a> {
    repo_name: &'a str,
}

impl<'a> UrlBuilder<'a> {
    /// The root URL of OpenDigger static data
    const ROOT: &str = "https://oss.x-lab.info/open_digger/github/";

    #[inline]
    pub fn new(repo_name: &'a str) -> Self {
        Self { repo_name }
    }

    #[inline]
    pub fn build_url_with_metric(&self, metric: &Metric) -> String {
        Self::ROOT.to_owned() + self.repo_name + "/" + &metric.to_string() + ".json"
    }
}
