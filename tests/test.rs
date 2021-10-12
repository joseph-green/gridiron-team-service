#[deny(warnings)]
#[allow(unused_imports)]
mod helpers;

#[cfg(test)]
mod test {

    use reqwest::blocking::{Client};
    use hyper::StatusCode;
    use uuid::Uuid;
    use crate::helpers::{create_test_team_for_user, TeamResponse};

    #[test]
    fn test_ok() -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::blocking::get("http://localhost:3030/")?;

        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }

    #[test]
    fn test_get_teams() -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::blocking::get("http://localhost:3030/teams")?;

        assert_eq!(res.status(), StatusCode::OK);

        let _teams: Vec<TeamResponse> = res.json().unwrap();

        assert!(_teams.len() > 0);
        assert_ne!(_teams[0].id,Uuid::nil());
        Ok(())
    }

    #[test]
    fn test_get_team() -> Result<(), Box<dyn std::error::Error>> {

        let _user_uuid = Uuid::new_v4();

        let to_get: Uuid = create_test_team_for_user(&_user_uuid);

        let res = reqwest::blocking::get(format!("http://localhost:3030/teams/{}",to_get))?;

        assert_eq!(res.status(), StatusCode::OK);

        let _team: TeamResponse = res.json().unwrap();

        assert_eq!(_team.id, to_get);
        assert_eq!(_team.user, _user_uuid);
        assert_eq!(_team.name, "Fantasy Team");
        assert_eq!(_team.cards.len(), 0);
        Ok(())
    }

    #[test]
    fn test_create_team() -> Result<(), Box<dyn std::error::Error>> {
        let _client = Client::new();

        let _user_uuid = Uuid::new_v4();

        let json_body = format!("{{ \"name\": \"Fantasy Team\", \"user\": \"{}\", \"cards\": [] }}",_user_uuid);

        let res = _client.post("http://localhost:3030/teams")
            .body(json_body)
            .send()?;

        assert_eq!(res.status(), StatusCode::CREATED);
        Ok(())
    }

    #[test]
    fn test_update_team() -> Result<(), Box<dyn std::error::Error>> {
        let _client = Client::new();

        let _user_uuid = Uuid::new_v4();
        let to_update: Uuid = create_test_team_for_user(&_user_uuid);

        let new_card = Uuid::new_v4();
        let new_team_name = "Other Fantasy Team";

        let json_body = format!("{{ \"name\": \"{}\", \"cards\": [\"{}\"] }}", new_team_name, new_card);

        let res = _client.patch(format!("http://localhost:3030/teams/{}",to_update))
            .body(json_body)
            .send()?;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        Ok(())
    }

    #[test]
    fn test_delete_team() -> Result<(), Box<dyn std::error::Error>> {
        let _client = Client::new();

        let _user_uuid = Uuid::new_v4();

        let to_delete: Uuid = create_test_team_for_user(&_user_uuid);

        let res = _client.delete(format!("http://localhost:3030/teams/{}",to_delete))
            .send()?;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        Ok(())
    }


}