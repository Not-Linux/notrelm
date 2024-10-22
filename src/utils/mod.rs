use std::fs::{read_to_string, write};
use serde::{Deserialize, Serialize};
use font_kit::source::SystemSource;

pub mod traits;

/// The path to the personalization configuration file.
pub const PERSONALIZATION_CONFIG: &str = concat!(env!("HOME"), "/.config/not-linux/personalization.toml");

/// List of required fonts to be checked for installation.
pub const REQUIRED_FONTS: &[&str] = &["C059", "NDot", "Nothing Font (5x7)"];

/// The `PersonalizationConfig` struct stores user preferences, 
/// such as the `FontType`.
///
/// The struct can be serialized and deserialized from TOML, making it
/// easy to save and load configurations.
///
/// # Fields
/// - `font_type`: The type of font selected by the user.
#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
#[readonly::make]
pub struct PersonalizationConfig {
    pub font_type: FontType,
}

impl PersonalizationConfig {
    /// Loads the personalization configuration from the configuration file.
    ///
    /// If the file is missing or corrupted, it creates a default configuration
    /// and writes it to the file. Logs errors if the file cannot be read or written.
    ///
    /// # Returns
    /// A `PersonalizationConfig` with the user's saved settings, or the default settings
    /// if the file could not be loaded.
    pub fn load() -> Self {
        if let Ok(toml) = read_to_string(PERSONALIZATION_CONFIG) {
            match toml::from_str(&toml) {
                Ok(config) => {
                    log::info!("Personalization config loaded successfully");
                    return config;
                },
                Err(e) => {
                    log::error!("Cannot load personalization config {}: {}", PERSONALIZATION_CONFIG, e);
                },
            }
        };

        let config = PersonalizationConfig::default();
        if let Err(e) = write(PERSONALIZATION_CONFIG, toml::to_string_pretty(&config).unwrap()) {
            log::error!("Cannot write to personalization config: {}: {}", PERSONALIZATION_CONFIG, e);
        }

        config
    }
}

/// Enum representing the available font types.
///
/// This can be extended to support more font options in the future.
///
/// # Variants
/// - `Dot`: A small, dot-matrix-style font.
/// - `Serif`: A serif font, represented by "C059".
#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum FontType {
    #[default]
    Dot,
    Serif,
}

impl FontType {
    /// Returns the corresponding font family name for each `FontType`.
    ///
    /// # Returns
    /// - `"Nothing Font (5x7)"` for `FontType::Dot`.
    /// - `"C059"` for `FontType::Serif`.
    pub fn to_font_family(self) -> &'static str {
        match self {
            FontType::Dot => "Nothing Font (5x7)",
            FontType::Serif => "C059",
        }
    }
}

/// A struct representing a screen size with a width and height.
///
/// # Fields
/// - `width`: The width of the screen.
/// - `height`: The height of the screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Checks if the required fonts are installed on the system.
///
/// This function goes through each font in the `REQUIRED_FONTS` list
/// and logs whether each one is installed on the system.
pub fn check_fonts() {
    log::info!("Checking required fonts...");

    let source = SystemSource::new();

    for font_name in REQUIRED_FONTS {
        let font = source.select_family_by_name(font_name);

        match font {
            Ok(_) => log::info!("Font '{font_name}' is installed."),
            Err(_) => log::error!("Font '{font_name}' is not installed."),
        }
    }
}
