use std::sync::mpsc;

use crate::actions;
use crate::actions::Actions;

#[derive(Clone, Debug)]
pub enum Command {
    GetSetting(i64),
    SetSetting(String, i64),
    ReloadSettings,
    SaveSettings,
    ApplySettings,
    CountRows(&'static str),
    GetFilePatterns,
    GetFileSpecs,
    DeleteFilePattern,
    CreateTempFileDatabase,
    RestoreFileDatabase,
    AddFilePattern,
    NoReturnSql,
    DeleteFromDatabase,
    ToggleLock(String),
    GenerateNewFileName,
    GetSettingsVersion,
}

#[derive(Clone, Debug)]
pub enum CommandReturn {
    GetSetting(String),
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
    ToggleLock(),
    GenerateNewFileName(String),
    GetSettingsVersion,
}

impl Command {
    pub fn handle(self, actions: crate::actions::Actions) -> Result<CommandReturn, anyhow::Error> {
        use Command::*;
        match self.clone() {
            AddFilePattern => Ok(CommandReturn::AddFilePattern),
            ApplySettings => Ok(CommandReturn::ApplySettings),
            CountRows(x) => count_rows(x.to_string(), actions),
            CreateTempFileDatabase => create_temp_file_database(),
            DeleteFilePattern => Ok(CommandReturn::DeleteFilePattern),
            DeleteFromDatabase => Ok(CommandReturn::DeleteFromDatabase),
            GenerateNewFileName => Ok(CommandReturn::GenerateNewFileName(String::from(""))),
            GetFilePatterns => Ok(CommandReturn::GetFilePatterns),
            GetFileSpecs => Ok(CommandReturn::GetFileSpecs),
            GetSetting(x) => get_setting(x, actions),
            GetSettingsVersion => Ok(CommandReturn::GetSettingsVersion),
            NoReturnSql => Ok(CommandReturn::NoReturnSql),
            ReloadSettings => Ok(CommandReturn::ReloadSettings),
            RestoreFileDatabase => Ok(CommandReturn::RestoreFileDatabase),
            SaveSettings => Ok(CommandReturn::SaveSettings),
            SetSetting(_, _) => Ok(CommandReturn::SetSetting),
            ToggleLock(path) => toggle_lock(path, actions),
        }
    }
}

fn add_file_pattern() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::AddFilePattern)
}

fn apply_settings() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::ApplySettings)
}

fn count_rows(table: String, actions: crate::actions::Actions) -> anyhow::Result<CommandReturn> {
    let rows = actions.count_rows(table.clone())?;
    Ok(CommandReturn::CountRows(rows))
}

fn create_temp_file_database() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::CreateTempFileDatabase)
}

fn delete_file_pattern() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::DeleteFilePattern)
}

fn delete_from_database() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::DeleteFromDatabase)
}

fn generate_new_file_name() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::GenerateNewFileName(String::from("")))
}

fn get_file_patterns() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::GetFilePatterns)
}

fn get_file_specs() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::GetFileSpecs)
}

fn get_setting(name: i64, actions: Actions) -> anyhow::Result<CommandReturn> {
    let value = actions.get_setting(name)?;
    Ok(CommandReturn::GetSetting(value.to_string()))
}

fn get_settings_version() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::GetSettingsVersion)
}

fn no_return_sql() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::NoReturnSql)
}

fn reload_settings() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::ReloadSettings)
}

fn restore_file_database() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::RestoreFileDatabase)
}

fn save_settings() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::SaveSettings)
}

fn set_setting() -> anyhow::Result<CommandReturn> {
    Ok(CommandReturn::SetSetting)
}

fn toggle_lock(path: String, actions: Actions) -> anyhow::Result<CommandReturn> {
    actions.toggle_lock(path)?;
    Ok(CommandReturn::ToggleLock())
}
