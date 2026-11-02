// Custom Skin: wear your own 64x64 skin PNG on the local player, in single
// player, on LAN and on servers (client side: only you see it). The mod
// reads a PNG from `horizon-mods/.horizon-mod-data/horizon_skin/skin.png`,
// or from whatever `/skin <file>` copied there, turns it into the atlas
// sprite the player model draws and keeps it applied every tick.
//
// This file is both the mod's Cargo library (its tests run with `cargo
// test`) and the module the bundle adds to the game as
// crate::game::horizon_skin.

use std::cell::{Cell, RefCell};
use std::time::SystemTime;

/// The mod's id (its row in Options > Mods and its settings folder), name,
/// version and one-line description.
pub const ID: &str = "horizon_skin";
pub const NAME: &str = "Custom Skin";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Wear your own skin PNG: /skin <file>, or leave skin.png in this mod's folder.";

/// The atlas sprite the skin lives under. The `skins/` prefix makes the
/// game treat it as a stored skin (the same place downloaded skins go).
pub const LOCATION: &str = "skins/horizon_skin";
/// The same skin under a name the game reads as a slim model
/// (`PlayerInfo.getSkin`: the `skin_of` fallback takes `slim` from the path).
pub const SLIM_LOCATION: &str = "skins/horizon_skin/slim/model";
/// The file read when the settings name none.
pub const FILE: &str = "skin.png";

/// How the arms are drawn: vanilla's classic (4 pixels) or slim (3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    Classic,
    Slim,
}

impl Model {
    pub fn read(value: Option<String>) -> Self {
        match value.as_deref().map(str::trim) {
            Some("slim") => Self::Slim,
            _ => Self::Classic,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Slim => "slim",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "CLASSIC",
            Self::Slim => "SLIM",
        }
    }
    pub fn slim(self) -> bool {
        self == Self::Slim
    }
    pub fn other(self) -> Self {
        match self {
            Self::Classic => Self::Slim,
            Self::Slim => Self::Classic,
        }
    }
}

/// The mod's two settings, one file each in its data folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// The mod's own switch, beside the one Options > Mods gives every mod.
    pub enabled: bool,
    pub model: Model,
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let enabled = get("enabled").is_none_or(|v| v.trim() != "false");
        Self {
            enabled,
            model: Model::read(get("model")),
        }
    }
    pub fn labels(&self) -> Vec<String> {
        let on = if self.enabled { "ON" } else { "OFF" };
        vec![
            format!("Custom skin: {on}"),
            format!("Arms: {}", self.model.label()),
            "Reload skin.png".to_owned(),
        ]
    }
    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.enabled = !self.enabled,
            1 => self.model = self.model.other(),
            _ => {}
        }
    }
    pub fn stored(&self) -> [(&'static str, String); 2] {
        [
            ("enabled", if self.enabled { "true" } else { "false" }.to_owned()),
            ("model", self.model.key().to_owned()),
        ]
    }
}

thread_local! {
    /// The settings, read once from the files, then kept (as the official
    /// mods do: a settings page and the tick share them).
    static SETTINGS: Cell<Option<Settings>> = const { Cell::new(None) };
    /// The decoded 64x64 RGBA of the last skin read, with the file it came
    /// from and its modification time, so the tick does not read the PNG
    /// twenty times a second.
    static PIXELS: RefCell<Option<Cached>> = const { RefCell::new(None) };
    /// Whether the mod's skin is the one the player model currently wears.
    static WEARING: Cell<bool> = const { Cell::new(false) };
    /// The last load error, so it is said in chat once, not every tick.
    static ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// One decoded skin, with what says it is still the file's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cached {
    pub path: String,
    pub stamp: Option<SystemTime>,
    pub rgba: Vec<u8>,
}

/// The settings, read once through `get` (one file per key).
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|s| {
        let found = s.get().unwrap_or_else(|| Settings::read(get));
        s.set(Some(found));
        found
    })
}

/// Replaces the settings (a command or the settings page changed them).
pub fn set_settings(settings: Settings) {
    SETTINGS.with(|s| s.set(Some(settings)));
}

/// The decoded skin of `path` if it was read at `stamp`.
pub fn cached_pixels(path: &str, stamp: Option<SystemTime>) -> Option<Vec<u8>> {
    PIXELS.with(|p| {
        let p = p.borrow();
        p.as_ref()
            .filter(|c| c.path == path && c.stamp == stamp)
            .map(|c| c.rgba.clone())
    })
}

/// Keeps a decoded skin for the next ticks.
pub fn remember_pixels(path: String, stamp: Option<SystemTime>, rgba: Vec<u8>) {
    PIXELS.with(|p| {
        *p.borrow_mut() = Some(Cached {
            path,
            stamp,
            rgba,
        })
    });
}

/// Reads the file again next tick (`/skin reload`, or a new file).
pub fn forget_pixels() {
    PIXELS.with(|p| *p.borrow_mut() = None);
}

/// Records that the player model wears (`true`) the mod's skin or not;
/// returns what it was.
pub fn set_wearing(on: bool) -> bool {
    WEARING.with(|w| w.replace(on))
}

/// Whether `error` is new (to say it in chat once, not every tick).
pub fn new_error(error: &str) -> bool {
    ERROR.with(|e| {
        let mut e = e.borrow_mut();
        if e.as_deref() == Some(error) {
            return false;
        }
        *e = Some(error.to_owned());
        true
    })
}

/// Forgets the last error (a reload may fix it silently).
pub fn clear_error() {
    ERROR.with(|e| *e.borrow_mut() = None);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn reads_settings_with_defaults_on() {
        assert_eq!(
            Settings::read(stored(&[])),
            Settings {
                enabled: true,
                model: Model::Classic
            }
        );
        assert_eq!(
            Settings::read(stored(&[("enabled", "false\n"), ("model", "slim ")])),
            Settings {
                enabled: false,
                model: Model::Slim
            }
        );
        // Anything but "false" is on, as the files only ever hold true/false.
        assert!(Settings::read(stored(&[("enabled", "yes")])).enabled);
    }

    #[test]
    fn the_page_toggles_and_stores() {
        let mut s = Settings {
            enabled: true,
            model: Model::Classic,
        };
        assert_eq!(
            s.labels(),
            ["Custom skin: ON", "Arms: CLASSIC", "Reload skin.png"]
        );
        s.click(0);
        s.click(1);
        assert_eq!(
            s,
            Settings {
                enabled: false,
                model: Model::Slim
            }
        );
        assert_eq!(
            s.stored(),
            [("enabled", "false".into()), ("model", "slim".into())]
        );
        assert_eq!(
            s.labels(),
            ["Custom skin: OFF", "Arms: SLIM", "Reload skin.png"]
        );
    }

    #[test]
    fn the_model_round_trips() {
        assert_eq!(Model::read(Some("slim".into())), Model::Slim);
        assert_eq!(Model::read(Some("classic".into())), Model::Classic);
        assert_eq!(Model::read(None), Model::Classic);
        assert_eq!(Model::Slim.key(), "slim");
        assert!(Model::Slim.slim());
        assert!(!Model::Classic.slim());
        assert_eq!(Model::Slim.other(), Model::Classic);
    }

    #[test]
    fn a_cached_skin_is_only_the_same_file() {
        let a = Cached {
            path: "a.png".into(),
            stamp: None,
            rgba: vec![1, 2, 3, 4],
        };
        remember_pixels(a.path.clone(), a.stamp, a.rgba.clone());
        assert_eq!(cached_pixels("a.png", None), Some(vec![1, 2, 3, 4]));
        assert_eq!(cached_pixels("b.png", None), None);
        forget_pixels();
        assert_eq!(cached_pixels("a.png", None), None);
    }

    #[test]
    fn an_error_is_said_once() {
        clear_error();
        assert!(new_error("no skin.png"));
        assert!(!new_error("no skin.png"));
        assert!(new_error("not a skin"));
        clear_error();
        assert!(new_error("no skin.png"));
    }
}
