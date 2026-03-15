use redb::Database;

use crate::channel::{FromApp, ToApp};
use crate::handler::Handler as _;

pub(crate) fn run_db_thread(db: Database, to_from_app: (ToApp, FromApp)) {
    run_inner(db, to_from_app).unwrap()
}

fn run_inner(
    mut db: Database,
    (to_app, from_app): (ToApp, FromApp),
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let req = from_app.recv()?;
        let rep = db.handle(req)?;
        to_app.send(rep)?;
    }
}
