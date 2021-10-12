use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use postgres::Row;

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Team {
    #[serde(skip_deserializing)]
    pub id: Uuid,

    #[serde(default)]
    pub user: Uuid,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub cards: Vec<Uuid>
}

impl Team {
    pub fn new(name: String, user: Uuid) -> Team {
        let _team = Team { id: Uuid::new_v4(), user, name, cards: Vec::new() };
        return _team;
    }
    pub fn from_row(_row: &Row) -> std::result::Result<Team, mobc::Error<Team>> {
        let _team = Team { id: _row.get("id"), user: _row.get("user"), name: _row.get("name"), cards: Vec::new()};
        Ok(_team)
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.id,self.name)
    }
}

impl Default for Team {
    fn default() -> Self {
        return Team {
            id: Uuid::new_v4(),
            user: Uuid::nil(),
            name: String::new(),
            cards: Vec::new()
        }
    }
}
