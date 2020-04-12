mod district_api;
mod event_api;
mod list_api;
mod match_api;
mod tba_api;
mod team_api;

pub use district_api::*;
pub use event_api::*;
pub use list_api::*;
pub use match_api::*;
pub use tba_api::*;
pub use team_api::*;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::rc::Rc;

    use crate::configuration::Configuration;
    use crate::apis::*;
    use crate::Error;

    fn get_or_panic(key: &str) -> String {
        env::var(key).expect(&format!("expected {} to be set!", key))
    }

    struct TestFacts {
        apikey: String,
        current_season: i32,
        event: String,
        event_num_teams: usize,
        district: String,
        district_num_teams: usize,
        team: String,
        team_event_num_matches: usize
    }

    impl TestFacts {
        fn load() -> TestFacts {
            TestFacts {
                apikey: get_or_panic("TBA_AUTH_KEY"),
                current_season: 2020,
                event: "2020vahay".to_owned(),
                event_num_teams: 38,
                district: "2020chs".to_owned(),
                district_num_teams: 127,
                team: "frc2363".to_owned(),
                team_event_num_matches: 12
            }
        }
    }

    #[tokio::test]
    async fn district_works() {
        let facts = TestFacts::load();
        let config = Rc::new(Configuration::from_api_key(facts.apikey.to_owned()));
        let district = DistrictApiClient::new(config.clone());

        let result = district.get_district_teams_simple(&facts.district, None).await.unwrap();
        assert_eq!(facts.district_num_teams, result.len());

        for team in result {
            println!("{} {}", team.nickname.unwrap_or_default(), team.team_number);
        }
    }

    #[tokio::test]
    async fn event_works() {
        let facts = TestFacts::load();
        let config = Rc::new(Configuration::from_api_key(facts.apikey.to_owned()));
        let api = EventApiClient::new(config.clone());

        let result = api.get_event_teams(&facts.event, None).await.unwrap();
        println!("{:?}", result);

        assert_eq!(facts.event_num_teams, result.len());

        for team in result {
            println!("{} {}", team.nickname.unwrap_or_default(), team.team_number);
        }
    }

    #[tokio::test]
    async fn list_works() {
        let facts = TestFacts::load();
        let config = Rc::new(Configuration::from_api_key(facts.apikey.to_owned()));
        let api = ListApiClient::new(config.clone());

        let result = api.get_event_teams(&facts.event, None).await.unwrap();
        println!("{:?}", result);

        assert_eq!(facts.event_num_teams, result.len());

        for team in result {
            println!("{} {}", team.nickname.unwrap_or_default(), team.team_number);
        }
    }

    #[tokio::test]
    async fn team_works() {
        let facts = TestFacts::load();
        let config = Rc::new(Configuration::from_api_key(facts.apikey.to_owned()));
        let api = TeamApiClient::new(config.clone());

        let result = api.get_team_event_matches(&facts.team, &facts.event, None).await.unwrap();
        println!("{:?}", result);

        assert_eq!(facts.team_event_num_matches, result.len());

        for team in result {
            println!("{} {}", team.nickname.unwrap_or_default(), team.team_number);
        }
    }

    #[tokio::test]
    async fn tba_works() {
        let facts = TestFacts::load();
        let config = Rc::new(Configuration::from_api_key(facts.apikey.to_owned()));
        let api = TBAApiClient::new(config.clone());

        let result = api.get_status(None).await.unwrap();
        println!("{:?}", result);

        assert_eq!(facts.current_season, result.current_season);
    }

}