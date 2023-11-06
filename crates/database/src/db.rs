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

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        Ok(self.execute(
            r#"
               DROP TABLE IF EXISTS files;
               CREATE TABLE files (
                    path TEXT NOT NULL UNIQUE, /* Full path to image file */
                    created DATETIME, /* The time file file was created in seconds since Unix epoc */ 
                    modified DATETIME, /* The time file file was modified in seconds since Unix epoc */ 
                    orig_file_name TEXT, 
                    new_file_name TEXT,
                    nksc_path TEXT, /* Path to the Nikon sidecar file */
                    inNXstudio BOOL DEFAULT -1, /* has an entry in the NX Studio sqlite database */
                    tmp_lock BOOL DEFAULT -1, /* Temporary lock for internal use */
                    locked BOOL DEFAULT -1 /* Name change manually locked */
                );

                DROP TABLE IF EXISTS exif;
                CREATE TABLE exif (
                    path TEXT NOT NULL, /* Full path to the original image file */
                    tag TEXT NOT NULL, /* An exif TAG shorhand in text, as opposed to ID */
                    tag_id,
                    value TEXT NOT NULL, /* The value of the exif tag */
                
                    UNIQUE(path,tag)
                );
            "#,
        )?)
    }
}
