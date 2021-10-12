use uuid::Uuid;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TeamResponse {
    pub id: Uuid,
    pub user: Uuid,
    pub name: String,
    pub cards: Vec<Uuid>
}

impl Default for TeamResponse {
    fn default() -> Self {
        return TeamResponse {
            id: Uuid::nil(),
            user: Uuid::nil(),
            name: String::new(),
            cards: Vec::new()
        }
    }
}
// creates a test team and returns its uuid
pub fn create_test_team_for_user(_user: &Uuid) -> Uuid {

    let _client = Client::new();

    let json_body = format!("{{ \"name\": \"Fantasy Team\", \"user\": \"{}\",  \"cards\": [] }}", _user);

    let res = _client.post("http://localhost:3030/teams")
        .body(json_body)
        .send()
        .unwrap();

    let _team: TeamResponse = res.json().unwrap();

    return _team.id;

}
