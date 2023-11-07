use std::sync::mpsc;

pub enum Command {
    GetSetting(String),
    SetSetting(String, i64),
    ReloadSettings,
    SaveSettings,
    ApplySettings,
    CountRows(&str),
    GetFilePatterns,
    GetFileSpecs,
    DeleteFilePattern,
    CreateTempFileDatabase,
    RestoreFileDatabase,
    AddFilePattern,
    NoReturnSql,
    DeleteFromDatabase,
    ToggleLock,
    GenerateNewFileName,
    GetSettingsVersion,
}

pub enum CommandReturn {
    GetSetting(i64),
    SetSetting,
    ReloadSettings,
    SaveSettings,
    ApplySettings,
    CountRows(i64),
    GetFilePatterns,
    GetFileSpecs,
    DeleteFilePattern,
    CreateTempFileDatabase,
    RestoreFileDatabase,
    AddFilePattern,
    NoReturnSql,
    DeleteFromDatabase,
    ToggleLock,
    GenerateNewFileName(String),
    GetSettingsVersion,
}

impl Command {
    fn handle(&self, actions: crate::actions::Actions) -> Result<CommandReturn, anyhow::Error> {
        use Command::*;
        match *self {
            AddFilePattern => Ok(CommandReturn::AddFilePattern),
            ApplySettings => Ok(CommandReturn::ApplySettings),
            CountRows(x) => count_rows(x, actions),
            CreateTempFileDatabase => Ok(CommandReturn::CreateTempFileDatabase),
            DeleteFilePattern => Ok(CommandReturn::DeleteFilePattern),
            DeleteFromDatabase => Ok(CommandReturn::DeleteFromDatabase),
            GenerateNewFileName => Ok(CommandReturn::GenerateNewFileName(String::from(""))),
            GetFilePatterns => Ok(CommandReturn::GetFilePatterns),
            GetFileSpecs => Ok(CommandReturn::GetFileSpecs),
            GetSetting(_) => Ok(CommandReturn::GetSetting(0)),
            GetSettingsVersion => Ok(CommandReturn::GetSettingsVersion),
            NoReturnSql => Ok(CommandReturn::NoReturnSql),
            ReloadSettings => Ok(CommandReturn::ReloadSettings),
            RestoreFileDatabase => Ok(CommandReturn::RestoreFileDatabase),
            SaveSettings => Ok(CommandReturn::SaveSettings),
            SetSetting(_, _) => Ok(CommandReturn::SetSetting),
            ToggleLock => Ok(CommandReturn::ToggleLock),
        }
    }
}

fn add_file_pattern() -> anyhow::Result<CommandReturn> {
    CommandReturn::AddFilePattern
}

fn apply_settings() -> anyhow::Result<CommandReturn> {
    CommandReturn::ApplySettings
}

fn count_rows(table: &str, actions: crate::actions::Actions) -> anyhow::Result<CommandReturn> {
    let rows = actions.count_rows(table)?;
    Ok(CommandReturn::CountRows(rows))
}

fn create_temp_file_database() -> anyhow::Result<CommandReturn> {
    CommandReturn::CreateTempFileDatabase
}

fn delete_file_pattern() -> anyhow::Result<CommandReturn> {
    CommandReturn::DeleteFilePattern
}

fn delete_from_database() -> anyhow::Result<CommandReturn> {
    CommandReturn::DeleteFromDatabase
}

fn generate_new_file_name() -> anyhow::Result<CommandReturn> {
    CommandReturn::GenerateNewFileName(String::from(""))
}

fn get_file_patterns() -> anyhow::Result<CommandReturn> {
    CommandReturn::GetFilePatterns
}

fn get_file_specs() -> anyhow::Result<CommandReturn> {
    CommandReturn::GetFileSpecs
}

fn get_setting() -> anyhow::Result<CommandReturn> {
    CommandReturn::GetSetting(0)
}

fn get_settings_version() -> anyhow::Result<CommandReturn> {
    CommandReturn::GetSettingsVersion
}

fn no_return_sql() -> anyhow::Result<CommandReturn> {
    CommandReturn::NoReturnSql
}

fn reload_settings() -> anyhow::Result<CommandReturn> {
    CommandReturn::ReloadSettings
}

fn restore_file_database() -> anyhow::Result<CommandReturn> {
    CommandReturn::RestoreFileDatabase
}

fn save_settings() -> anyhow::Result<CommandReturn> {
    CommandReturn::SaveSettings
}

fn set_setting() -> anyhow::Result<CommandReturn> {
    CommandReturn::SetSetting
}

fn toggle_lock() -> anyhow::Result<CommandReturn> {
    CommandReturn::ToggleLock
}
