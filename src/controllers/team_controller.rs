use std::convert::Infallible;
use crate::models::team::Team;
use hyper::StatusCode;
use warp::reply::{with_status, json};
use crate::{DBPool, DBCon};
use crate::db::get_connection_from_pool;
use uuid::Uuid;
use std::str::FromStr;


// GET /teams
pub async fn list_teams(_pool: DBPool) -> Result<impl warp::Reply, Infallible> {
    log::info!("GET TEAMS");
    let _con: DBCon = get_connection_from_pool(&_pool).await.unwrap();

    let _rows = _con.query("SELECT * FROM public.teams",&[]).await.unwrap() as Vec<tokio_postgres::row::Row>;

    let mut _teams: Vec<Team> = Vec::new();

    for _row in _rows {
        _teams.push(Team::from_row(&_row).unwrap());
    }

    return Ok(json(&_teams));

}

// GET /teams/[id]
pub async fn get_team(_uuid: String, _pool: DBPool) -> Result<impl warp::Reply, Infallible> {
    log::info!("{}", format!("GET TEAM {}", _uuid));
    let _con: DBCon = get_connection_from_pool(&_pool).await.unwrap();

    let _row = _con.query_one("SELECT * FROM public.teams WHERE id = $1",&[&Uuid::from_str(&_uuid).unwrap()])
        .await.unwrap();

    let _team: Team = Team::from_row(&_row).unwrap();

    return Ok(json(&_team));
}

// POST /teams
pub async fn create_team(_team: Team, _pool: DBPool) -> Result<impl warp::Reply, Infallible> {
    log::info!("{}", format!("CREATE TEAM {}", _team));
    let _con: DBCon = get_connection_from_pool(&_pool).await.unwrap();
    _con.execute("INSERT INTO public.teams(id,\"user\",name) \
                            VALUES($1,$2,$3)",&[&_team.id,&_team.user,&_team.name]).await.unwrap();
    return Ok(with_status(json(&_team),StatusCode::CREATED));
}

// PATCH /teams/[id]
pub async fn update_team(_uuid: String, _team: Team, _pool: DBPool) -> Result<impl warp::Reply, Infallible> {
    log::info!("{}", format!("UPDATE TEAM {}", _uuid));
    let _con: DBCon = get_connection_from_pool(&_pool).await.unwrap();
    _con.execute("UPDATE public.teams SET name = $1 WHERE id = $2",&[&_team.name,&Uuid::from_str(&_uuid).unwrap()]).await.unwrap();
    return Ok(with_status(json(&String::new()),StatusCode::NO_CONTENT));
}

// DELETE /teams/[id]
pub async fn delete_team(_uuid: String, _pool: DBPool) -> Result<impl warp::Reply, Infallible> {
    log::info!("{}", format!("DELETE TEAM {}", _uuid));
    let _con: DBCon = get_connection_from_pool(&_pool).await.unwrap();
    _con.execute("DELETE FROM public.teams WHERE id = $1",&[&Uuid::from_str(&_uuid).unwrap()]).await.unwrap();
    return Ok(with_status(json(&String::new()),StatusCode::NO_CONTENT));
}

