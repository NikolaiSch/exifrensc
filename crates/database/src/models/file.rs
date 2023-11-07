use std::fmt;

use rusqlite::ToSql;

#[derive(Clone, PartialEq)]
pub struct File {
    pub path: String,
    pub created: i64,
    pub modified: i64,
    pub orig_file_name: String,
    pub new_file_name: String,
    pub nksc_path: String,
    pub in_nx_studio: bool,
    pub tmp_lock: bool,
    pub locked: bool,
}

impl Default for File {
    fn default() -> Self {
        File {
            path: String::new(),
            created: 0,
            modified: 0,
            orig_file_name: String::new(),
            new_file_name: String::new(),
            nksc_path: String::new(),
            in_nx_studio: false,
            tmp_lock: false,
            locked: false,
        }
    }
}

impl File {
    pub fn new(
        path: &str,
        created: i64,
        modified: i64,
        orig_file_name: &str,
        new_file_name: &str,
        nksc_path: &str,
        in_nx_studio: bool,
        tmp_lock: bool,
        locked: bool,
    ) -> File {
        File {
            path: path.to_string(),
            created,
            modified,
            orig_file_name: orig_file_name.to_string(),
            new_file_name: new_file_name.to_string(),
            nksc_path: nksc_path.to_string(),
            in_nx_studio,
            tmp_lock,
            locked,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}, created: {}, modified: {}, orig_file_name: {}, new_file_name: {}, nksc_path: {}, in_nx_studio: {}, tmp_lock: {}, locked: {}",
            self.path, self.created, self.modified, self.orig_file_name, self.new_file_name, self.nksc_path, self.in_nx_studio, self.tmp_lock, self.locked
        )
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}, created: {}, modified: {}, orig_file_name: {}, new_file_name: {}, nksc_path: {}, in_nx_studio: {}, tmp_lock: {}, locked: {}",
            self.path, self.created, self.modified, self.orig_file_name, self.new_file_name, self.nksc_path, self.in_nx_studio, self.tmp_lock, self.locked
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_default() {
        let file = File::default();
        assert_eq!(file.path, "");
        assert_eq!(file.created, 0);
        assert_eq!(file.modified, 0);
        assert_eq!(file.orig_file_name, "");
        assert_eq!(file.new_file_name, "");
        assert_eq!(file.nksc_path, "");
        assert_eq!(file.in_nx_studio, false);
        assert_eq!(file.tmp_lock, false);
        assert_eq!(file.locked, false);
    }

    #[test]
    fn test_file_new() {
        let file = File::new(
            "path",
            1,
            2,
            "orig_file_name",
            "new_file_name",
            "nksc_path",
            true,
            true,
            true,
        );
        assert_eq!(file.path, "path");
        assert_eq!(file.created, 1);
        assert_eq!(file.modified, 2);
        assert_eq!(file.orig_file_name, "orig_file_name");
        assert_eq!(file.new_file_name, "new_file_name");
        assert_eq!(file.nksc_path, "nksc_path");
        assert_eq!(file.in_nx_studio, true);
        assert_eq!(file.tmp_lock, true);
        assert_eq!(file.locked, true);
    }

    #[test]
    fn test_file_fmt_display() {
        let file = File::new(
            "path",
            1,
            2,
            "orig_file_name",
            "new_file_name",
            "nksc_path",
            true,
            true,
            true,
        );
        let s = format!("{}", file);
        assert_eq!(
            s,
            "path: path, created: 1, modified: 2, orig_file_name: orig_file_name, new_file_name: new_file_name, nksc_path: nksc_path, in_nx_studio: true, tmp_lock: true, locked: true"
        );
    }

    #[test]
    fn test_file_fmt_debug() {
        let file = File::new(
            "path",
            1,
            2,
            "orig_file_name",
            "new_file_name",
            "nksc_path",
            true,
            true,
            true,
        );
        let s = format!("{:?}", file);
        assert_eq!(
            s,
            "path: path, created: 1, modified: 2, orig_file_name: orig_file_name, new_file_name: new_file_name, nksc_path: nksc_path, in_nx_studio: true, tmp_lock: true, locked: true"
        );
    }
}
