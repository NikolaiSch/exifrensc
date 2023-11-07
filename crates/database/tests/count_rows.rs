use database::commands::Command;
use database::commands::CommandReturn;
use database::*;
use models::file::File;

fn main() {
    let db = database::db::Database::new().unwrap();
    let actions = database::actions::Actions::new(db);
    actions.create_tables().unwrap();
    actions.create_settings().unwrap();

    actions.db.insert_file(&File::default()).unwrap();

    if let CommandReturn::CountRows(x) = Command::CountRows("files").handle(actions).unwrap() {
        assert_eq!(1, x)
    }
}
