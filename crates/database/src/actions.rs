pub struct Actions {
    db: super::db::Database,
}

impl Actions {
    pub fn new(conn: super::db::Database) -> Actions {
        Actions { db: conn }
    }

    pub fn create_tables(&self) -> Result<(), rusqlite::Error> {
        let command = 
            "CREATE TABLE files (
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
                
            CREATE TABLE exif (
                path TEXT NOT NULL, /* Full path to the original image file */
                tag TEXT NOT NULL, /* An exif TAG shorhand in text, as opposed to ID */
                tag_id,
                value TEXT NOT NULL, /* The value of the exif tag */
            
                UNIQUE(path,tag)
            );";
        self.db.execute(&command)
    }

    pub fn drop_tables(&self) -> Result<(), rusqlite::Error> {
        let command = 
            "DROP TABLE IF EXISTS files;
            DROP TABLE IF EXISTS exif;";
        self.db.execute(&command)
    }

}
