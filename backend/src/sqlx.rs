use std::env;
use std::time::Duration;
use sqlx::postgres::PgPool;
use rocket::{Build, fairing, Rocket};
use rocket::fairing::AdHoc;
use tokio::time::{timeout};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
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

async fn connect_to_database(db_url: &str) -> Result<PgPool, String> {
    println!("Connecting to Database...");
    let timeout_duration = Duration::from_secs(10);
    match timeout(timeout_duration, PgPool::connect(db_url)).await {
        Ok(pool) => Ok(pool.expect("Could not connect to database! Check your connection!")),
        Err(_) => Err("Failed to connect to the database within the timeout period!".to_string()),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        //DATABASE_URL=postgres://monkey_user:monkey_pass@localhost/monkey_db
        let db_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let db_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
        let db_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let db_database = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        let db_url = format!("postgres://{}:{}@{}/{}", db_user, db_pass, db_host, db_database);
        println!("{}", db_url);

        match connect_to_database(&db_url).await {
            Ok(pool) => {
                let postgres_db = Db(pool);
                println!("Connected to the database!");
                rocket.manage(postgres_db)
                    .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            }
            Err(err) => {
                eprintln!("Failed to connect to the database: {}", err);
                std::process::exit(1);
            }
        }
    })
}