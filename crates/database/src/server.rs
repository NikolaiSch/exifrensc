use tokio::sync::mpsc;

use crate::commands::Command;

pub struct Server {
    pub actions: actions::Actions,
    channel: mpsc::Receiver<Command>,
    sender: mpsc::Sender<Command>,
}

impl Server {
    pub fn create_db_server() -> Result<Server, Error> {
        let db = db::Database::new()?;
        let actions = actions::Actions::new(db);
        actions.create_tables()?;
        actions.create_settings()?;
        actions.attach_settings(PathBuf::from("settings.db"))?;
        Ok(Server::new(actions))
    }

    pub fn new(actions: actions::Actions) -> Server {
        let (tx, rx) = mpsc::channel(24);
        Server {
            actions,
            channel: (rx),
            sender: (tx),
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Command> {
        self.sender.clone()
    }
}
