use database::commands::Command;
use database::commands::CommandReturn;
use database::*;
use models::file::File;

fn main() {
    // create settings database
    let settings_db = db::Database::new_settings().unwrap();
    let settings_actions = actions::Actions::new(settings_db);
    settings_actions.create_settings().unwrap();
}
