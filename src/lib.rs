pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::Event;
use std::env;

use models::NewEvent;

/// Establishes a connection to the SQLite database
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Fetches events from the database
pub fn get_events(conn: &mut SqliteConnection) -> Vec<Event> {
    use crate::schema::events::dsl::*;

    events
        .select(Event::as_select())
        .load(conn)
        .expect("Error loading posts")
}

/// Create an event and insert it into the database
pub fn create_event(conn: &mut SqliteConnection, title: &str, date: &str, description: &str) {
    use crate::schema::events;

    let new_event = NewEvent {
        title,
        date,
        description,
    };

    let _ = diesel::insert_into(events::table)
        .values(&new_event)
        .execute(conn)
        .expect("Error creating event");
}
