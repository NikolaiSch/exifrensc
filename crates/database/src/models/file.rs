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
