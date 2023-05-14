use std::env;
use rocket::{Rocket, Build};
use rocket::fairing::{self, AdHoc};

use rocket_db_pools::{sqlx, Database};
use sqlx::{PgPool};

#[derive(Database)]
#[database("monkey_db")]
pub struct Db(PgPool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        }
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&db_url).await.expect("Failed to connect to database");
        let postgres_db = Db(pool);

        rocket.manage(postgres_db)
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}