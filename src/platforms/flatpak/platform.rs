use serde::{Deserialize, Serialize};

use crate::platforms::{NeedsPorton, to_shortcuts, ShortcutToImport, GamesPlatform, FromSettingsString, load_settings};

use super::FlatpakSettings;
use steam_shortcuts_util::{shortcut::ShortcutOwned, Shortcut};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlatpakPlatform {
    pub settings: FlatpakSettings,
}

#[derive(Debug, Clone)]
pub struct FlatpakApp {
    pub name: String,
    pub id: String,
}

impl From<FlatpakApp> for ShortcutOwned {
    fn from(app: FlatpakApp) -> Self {
        let launch_parameter = format!("run {}", app.id);
        Shortcut::new("0", &app.name, "flatpak", "", "", "", &launch_parameter).to_owned()
    }
}

impl NeedsPorton<FlatpakPlatform> for FlatpakApp{
    fn needs_proton(&self, _platform: &FlatpakPlatform) -> bool {
        false
    }

    fn create_symlinks(&self, _platform: &FlatpakPlatform) -> bool {
        false
    }
}

impl FlatpakPlatform {

    fn get_flatpak_apps(&self) -> eyre::Result<Vec<FlatpakApp>> {
        use std::process::Command;
        let mut command = Command::new("flatpak");
        let output = command
            .arg("list")
            .arg("--app")
            .arg("--columns=name,application")
            .output()?;
        let output_string = String::from_utf8_lossy(&output.stdout).to_string();
        let mut result = vec![];
        for line in output_string.lines() {
            let mut split = line.split('\t');
            if let Some(name) = split.next() {
                if let Some(id) = split.next() {
                    result.push(FlatpakApp {
                        name: name.to_string(),
                        id: id.to_string(),
                    })
                }
            }
        }
        Ok(result)
    }
}


impl FromSettingsString for FlatpakPlatform{
    fn from_settings_string<S: AsRef<str>>(s: S) -> Self {
        FlatpakPlatform {
            settings: load_settings(s),
        }
    }
} 

impl GamesPlatform for FlatpakPlatform{
    fn name(&self) -> &str {
        "Flatpak"
    }

    fn enabled(&self) -> bool {
        self.settings.enabled
    }

    fn get_shortcut_info(&self) -> eyre::Result<Vec<ShortcutToImport>> {
        to_shortcuts(self, self.get_flatpak_apps())
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Flatpak");
        ui.checkbox(&mut self.settings.enabled, "Import from Flatpak");
    }
}