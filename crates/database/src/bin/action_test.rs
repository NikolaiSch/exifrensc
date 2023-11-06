use database::*;

fn main() {
    let db = database::db::Database::new_development().unwrap();
    let actions = database::actions::Actions::new(db);
    actions.create_tables().unwrap();
}
