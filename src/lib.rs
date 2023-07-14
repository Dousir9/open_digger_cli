mod errors;
mod url_builder;
mod utils;

pub use errors::{CliError, Result};
pub use url_builder::UrlBuilder;
use utils::camel_to_snake_case;
use clap::{ValueEnum, builder::PossibleValue};

/// The root URL of OpenDigger static data
const ROOT: &str = "https://oss.x-lab.info/open_digger/github/";

enum_with_to_string!{
    pub enum Metric {
        ActiveDatesAndTimes,
        Stars,
        TechnicalFork,
        Participants,
        NewContributors,
        NewContributorsDetail,
        InactiveContributors,
        BusFactor,
        BusFactorDetail,
        IssuesNew,
        IssuesClosed,
        IssueComments,
        IssueResponseTime,
        IssueResolutionDuration,
        IssueAge,
        CodeChangeLinesAdd,
        CodeChangeLinesRemove,
        CodeChangeLinesSum,
        ChangeRequests,
        ChangeRequestsAccepted,
        ChangeRequestsReviews,
        ChangeRequestResponseTime,
        ChangeRequestResolutionDuration,
        ChangeRequestAge,
        ActivityDetails,
    } 
}

#[cfg(test)]
mod tests {
    use crate::{Metric, UrlBuilder};

    #[test]
    fn test_enum() {
        let metric = Metric::ActiveDatesAndTimes;
        assert_eq!(metric.to_string().as_str(), "active_dates_and_time");
    }

    #[test]
    fn test_url_builder() {
        let repo_name = "test/test";
        let builder = UrlBuilder::new(repo_name).with_metric(Metric::BusFactor);
        let url = builder.build().unwrap();
        assert_eq!(url.as_str(), "https://oss.x-lab.info/open-digger/github/test/test/bus_factor.json");
    }
}