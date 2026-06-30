use iztro_gui::app::{Message, StaticChartApp};
use iztro_gui::settings::{GuiThemeId, SettingsStore};

#[test]
fn set_theme_updates_settings() {
    let mut app = StaticChartApp::new();
    assert_eq!(app.settings().theme, GuiThemeId::InkPaper);
    app.update(Message::SetTheme(GuiThemeId::JadeLight));
    assert_eq!(app.settings().theme, GuiThemeId::JadeLight);
    app.update(Message::SetTheme(GuiThemeId::DeepInk));
    assert_eq!(app.settings().theme, GuiThemeId::DeepInk);
    app.update(Message::SetTheme(GuiThemeId::InkPaper));
    assert_eq!(app.settings().theme, GuiThemeId::InkPaper);
}

#[test]
fn set_theme_persists_through_settings_store() {
    let dir = tempfile::tempdir().expect("temp dir");
    let store = SettingsStore::new(dir.path().join("settings.json"));
    let mut app = StaticChartApp::with_optional_stores(None, Some(store.clone()));
    app.update(Message::SetTheme(GuiThemeId::JadeLight));
    assert_eq!(store.load().theme, GuiThemeId::JadeLight);
}
