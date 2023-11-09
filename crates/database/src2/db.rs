use crate::actions::Actions;
use crate::models;

pub struct Database {
    pub conn: rusqlite::Connection,
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

    pub fn new_settings() -> Result<Database, rusqlite::Error> {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create("C:\\Users\\Public\\Documents\\exifrensc")
            .unwrap();

        let conn =
            rusqlite::Connection::open("C:\\Users\\Public\\Documents\\exifrensc\\settings.db")?;

        Ok(Database {
            conn: rusqlite::Connection::open(
                "C:\\Users\\Public\\Documents\\exifrensc\\settings.db",
            )?,
        })
    }

    pub fn execute(&self, command: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(command, [])?;
        Ok(())
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        Ok(self.conn.execute_batch(
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

    pub fn drop_table(&self) -> Result<(), rusqlite::Error> {
        Ok(self.execute(
            r#"
                DROP TABLE IF EXISTS files;
                DROP TABLE IF EXISTS exif;
            "#,
        )?)
    }

    pub fn insert_file(&self, file: &models::file::File) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            r#"
                INSERT INTO files (
                    path,
                    created,
                    modified,
                    orig_file_name,
                    new_file_name,
                    nksc_path,
                    inNXstudio,
                    tmp_lock,
                    locked
                ) VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9
                );
            "#,
            [
                &file.path,
                &file.created.to_string(),
                &file.modified.to_string(),
                &file.orig_file_name,
                &file.new_file_name,
                &file.nksc_path,
                &file.in_nx_studio.to_string(),
                &file.tmp_lock.to_string(),
                &file.locked.to_string(),
            ],
        )?;

        Ok(())
    }

    pub fn insert_files(&self, files: &Vec<models::file::File>) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            r#"
                INSERT OR REPLACE INTO files (
                    path,
                    created,
                    modified,
                    orig_file_name,
                    new_file_name,
                    nksc_path,
                    inNXstudio,
                    tmp_lock,
                    locked
                ) VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9
                );
            "#,
        )?;

        files.into_iter().for_each(|file| {
            stmt.execute([
                &file.path,
                &file.created.to_string(),
                &file.modified.to_string(),
                &file.orig_file_name,
                &file.new_file_name,
                &file.nksc_path,
                &file.in_nx_studio.to_string(),
                &file.tmp_lock.to_string(),
                &file.locked.to_string(),
            ]);
        });

        Ok(())
    }

    pub fn insert_exif(&self, exif: &models::exif::Exif) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            r#"
                INSERT INTO exif (
                    path,
                    tag,
                    tag_id,
                    value
                ) VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4
                );
            "#,
            [&exif.path, &exif.tag, &exif.tag_id.to_string(), &exif.value],
        )?;

        Ok(())
    }

    pub fn insert_exifs(&self, exifs: &Vec<models::exif::Exif>) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            r#"
                INSERT OR REPLACE INTO exif (
                    path,
                    tag,
                    tag_id,
                    value
                ) VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4
                );
            "#,
        )?;

        exifs.into_iter().for_each(|exif| {
            stmt.execute([&exif.path, &exif.tag, &exif.tag_id.to_string(), &exif.value]);
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_new() {
        Database::new().unwrap();
    }

    #[test]
    fn test_database_new_development() {
        Database::new_development().unwrap();

        if std::fs::File::open("development.db").is_ok() {
            std::fs::remove_file("development.db").unwrap();
        }
    }

    #[test]
    fn test_database_execute() {
        let database = Database::new().unwrap();
        let command = "CREATE TABLE test (id INTEGER PRIMARY KEY)";
        database.execute(command).unwrap();
    }

    #[test]
    fn test_database_create_table() {
        let database = Database::new().unwrap();
        database.create_table().unwrap();
    }

    #[test]
    fn test_database_drop_table() {
        let database = Database::new().unwrap();
        database.drop_table().unwrap();
    }

    #[test]
    fn test_database_insert_file() {
        let database = Database::new().unwrap();
        database.create_table().unwrap();
        let file = models::file::File::default();
        database.insert_file(&file).unwrap();

        let mut stmt = database
            .conn
            .prepare("SELECT * FROM files WHERE path = ?1")
            .unwrap();

        let mut rows = stmt.query([&file.path]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let path: String = row.get(0).unwrap();
            assert_eq!(path, file.path);
        }

        database.drop_table().unwrap();
    }

    #[test]
    fn test_database_insert_exif() {
        let database = Database::new().unwrap();
        database.create_table().unwrap();
        let exif = models::exif::Exif::default();
        database.insert_exif(&exif).unwrap();

        let mut stmt = database
            .conn
            .prepare("SELECT * FROM exif WHERE path = ?1")
            .unwrap();

        let mut rows = stmt.query([&exif.path]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let path: String = row.get(0).unwrap();
            assert_eq!(path, exif.path);
        }

        database.drop_table().unwrap();
    }
}
