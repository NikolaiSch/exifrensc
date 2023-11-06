pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn new() -> Result<Database, rusqlite::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        Ok(Database { conn })
    }

    pub fn new_development() -> Result<Database, rusqlite::Error> {
        let conn = rusqlite::Connection::open("development.db")?;
        Ok(Database { conn })
    }

    pub fn execute(&self, command: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(command, [])?;
        Ok(())
    }
}
