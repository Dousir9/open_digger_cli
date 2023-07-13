mod errors;
mod url_builder;
mod utils;

pub use errors::{CliError, Result};
pub use url_builder::UrlBuilder;
use utils::camel_to_snake_case;

/// The root URL of OpenDigger static data
const root: &str = "https://oss.x-lab.info/open-digger/github/";

enum_with_to_string!{
    pub enum Metric {
        ActiveDatesAndTime,
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
    use crate::Metric;

    #[test]
    fn test_enum() {
        let metric = Metric::ActiveDatesAndTime;
        assert_eq!(metric.to_string().as_str(), "active_dates_and_time");
    }
}