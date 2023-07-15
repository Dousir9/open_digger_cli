use clap::{builder::PossibleValue, ValueEnum};

use crate::errors::{CliError, Result};

pub(crate) fn camel_to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    let mut prev_char: Option<char> = None;

    for c in camel_case.chars() {
        if let Some(_) = prev_char {
            if c.is_uppercase() {
                snake_case.push('_');
                snake_case.push(c.to_lowercase().next().unwrap());
            } else {
                snake_case.push(c);
            }
            prev_char = Some(c);
        } else {
            snake_case.push(c.to_lowercase().next().unwrap());
            prev_char = Some(c);
        }
    }

    snake_case
}

#[macro_export]
macro_rules! execute_if {
    ($cond:expr, $expr:expr) => {
        if $cond {
            $expr
        }
    };
}

#[macro_export]
macro_rules! enum_with_to_string {
    (pub enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[derive(Clone)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn to_string(&self) -> String {
                match self {
                    $($name::$variant => camel_to_snake_case(stringify!($variant))),*
                }
            }

            pub fn valid_metrics() -> Vec<Self> {
                vec![
                    $($name::$variant),*
                ]
            }
        }

        impl ValueEnum for $name {
            fn value_variants<'a>() -> &'a [Self] {
                &[$($name::$variant),*]
            }

            fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
                Some(match self {
                    $($name::$variant => {
                        PossibleValue::new(stringify!($variant))
                    }),*
                })
            }
        }

        impl std::str::FromStr for $name {
            type Err = CliError;

            fn from_str(s: &str) -> Result<Self> {
                $(execute_if!(s.to_lowercase() == stringify!($variant).to_lowercase(), return Ok($name::$variant));)*
                return Err(CliError::String(format!("invalid variant: {s}")));
            }
        }
    };
}

enum_with_to_string! {
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
    use crate::metric::Metric;
    use crate::url_builder::UrlBuilder;

    #[test]
    fn test_enum() {
        let metric = Metric::ActiveDatesAndTimes;
        assert_eq!(metric.to_string().as_str(), "active_dates_and_time");
    }

    #[test]
    fn test_url_builder() {
        let repo_name = "test/test";
        let url_builder = UrlBuilder::new(repo_name);
        let url = url_builder.build_url_with_metric(&Metric::BusFactor);
        assert_eq!(
            url.as_str(),
            "https://oss.x-lab.info/open-digger/github/test/test/bus_factor.json"
        );
    }
}
