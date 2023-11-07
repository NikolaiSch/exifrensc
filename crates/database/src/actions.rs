use std::path::PathBuf;

use crate::models::file::File;
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

    pub fn create_settings(&self) -> Result<(), rusqlite::Error> {
          
        let cmd = format!(
            r#"
            CREATE TABLE IF NOT EXISTS 'settings' (name,ID,value);
            CREATE TABLE IF NOT EXISTS 'file_pat' 
              (
                idx INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, 
                pszName TEXT,
                pszSpec TEXT
              );
            CREATE TABLE IF NOT EXISTS version (version);  
           "#, 
            );
        
        self.db.conn.execute_batch(&cmd).expect("create_settings() failed.");
              Ok(())
    }
    
    pub fn drop_settings(&self) -> Result<(), rusqlite::Error> {
        let cmd = "
            DROP TABLE IF EXISTS 'settings';
            DROP TABLE IF EXISTS file_pat;
            DROP TABLE IF EXISTS version;";
        self.db.conn.execute_batch(&cmd)
    }

    pub fn attach_settings(&self, settings_path: PathBuf) -> Result<(), rusqlite::Error> {
        let cmd = format!("ATTACH DATABASE '{}' AS settings;", settings_path.to_str().unwrap());
        self.db.conn.execute_batch(&cmd)
    }

    pub fn count_rows(&self, table: &str) -> Result<i64, rusqlite::Error> {
        let cmd = format!("SELECT COUNT(*) FROM {}", table);
        let mut stmt = self.db.conn.prepare(&cmd)?;
        let mut rows = stmt.query([])?;
        let row = rows.next()?.unwrap();
        let count: i64 = row.get(0)?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::actions::Actions;

    use super::super::*;

    #[test]
    fn test_actions_new() {
        let db = db::Database::new().unwrap();
        actions::Actions::new(db);
        
    }

    #[test]
    fn test_actions_create_tables() {
        let db = super::super::db::Database::new().unwrap();
        let actions = Actions::new(db);
        assert_eq!(actions.create_tables(), Ok(()));
    }

    #[test]
    fn test_actions_drop_tables() {
        let db = super::super::db::Database::new().unwrap();
        let actions = Actions::new(db);
        assert_eq!(actions.drop_tables(), Ok(()));
    }

    #[test]
    fn test_actions_create_settings() {
        let db = super::super::db::Database::new_settings().unwrap();
        let actions = Actions::new(db);
        assert_eq!(actions.create_settings(), Ok(()));
    }

    #[test]
    fn test_actions_drop_settings() {
        let db = super::super::db::Database::new_settings().unwrap();
        let actions = Actions::new(db);
        assert_eq!(actions.drop_settings(), Ok(()));
    }

    #[test]
    fn test_actions_attach_settings() {
        let db = super::super::db::Database::new_settings().unwrap();
        let actions = Actions::new(db);
        actions.create_settings().unwrap();
        assert_eq!(actions.attach_settings(PathBuf::from("settings.db")), Ok(()));
    }
    
}