use self::models::*;
use diesel::prelude::*;
use kani_chan::*;

pub fn get_events() -> Vec<Event> {
    use self::schema::events::dsl::*;

    let connection = &mut establish_connection();

    events
        .select(Event::as_select())
        .load(connection)
        .expect("Error loading posts")
}
