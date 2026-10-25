//! Languages of the app. English is the default; German and Spanish are
//! complete translations. Texts are written inline with the `tr!` macro:
//! `tr!(lang, "English", "Deutsch", "Español")`.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Hash)]
pub enum Sprache {
    #[default]
    En,
    De,
    Es,
}

/// Picks the text for the given language.
#[macro_export]
macro_rules! tr {
    ($l:expr, $en:expr, $de:expr, $es:expr $(,)?) => {
        match $l {
            $crate::i18n::Sprache::En => $en,
            $crate::i18n::Sprache::De => $de,
            $crate::i18n::Sprache::Es => $es,
        }
    };
}

const SPEICHER: &str = "nrzlearn.lang";

impl Sprache {
    pub const ALLE: [Self; 3] = [Self::En, Self::De, Self::Es];

    pub const fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::De => "de",
            Self::Es => "es",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::De => "Deutsch",
            Self::Es => "Español",
        }
    }

    pub fn aus_code(code: &str) -> Option<Self> {
        let c = code.get(..2)?.to_ascii_lowercase();
        Self::ALLE.into_iter().find(|s| s.code() == c)
    }

    /// Saved choice in this browser, otherwise the browser language.
    pub fn aus_browser() -> Option<Self> {
        let w = web_sys::window()?;
        if let Some(s) = w.local_storage().ok().flatten().and_then(|s| s.get_item(SPEICHER).ok().flatten()) {
            return Self::aus_code(&s);
        }
        w.navigator().language().and_then(|l| Self::aus_code(&l))
    }

    pub fn merken(self) {
        if let Some(s) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = s.set_item(SPEICHER, self.code());
        }
        document::eval(&format!("document.documentElement.lang='{}'", self.code()));
    }
}

pub fn use_sprache() -> Sprache {
    use_context::<Signal<Sprache>>()()
}

pub fn use_sprache_signal() -> Signal<Sprache> {
    use_context::<Signal<Sprache>>()
}

/// Translated message for an API error code.
pub fn fehlertext(l: Sprache, code: &str) -> &'static str {
    use nrzlearn_shared::code as c;
    match code {
        c::INVALID_CREDENTIALS => tr!(l, "Wrong username or password.", "Benutzername oder Passwort falsch.", "Usuario o contraseña incorrectos."),
        c::USERNAME_TAKEN => tr!(l, "This username is already taken.", "Dieser Benutzername ist schon vergeben.", "Este nombre de usuario ya está en uso."),
        c::INVALID_USERNAME => tr!(
            l,
            "Usernames have 3–24 characters: letters, digits, . _ -",
            "Benutzernamen haben 3–24 Zeichen: Buchstaben, Ziffern, . _ -",
            "Los nombres de usuario tienen 3–24 caracteres: letras, cifras, . _ -"
        ),
        c::WEAK_PASSWORD => tr!(l, "The password needs at least 8 characters.", "Das Passwort braucht mindestens 8 Zeichen.", "La contraseña necesita al menos 8 caracteres."),
        c::REGISTRATION_CLOSED => tr!(l, "Registration is closed on this instance.", "Die Registrierung ist auf dieser Instanz geschlossen.", "El registro está cerrado en esta instancia."),
        c::SETUP_DONE => tr!(l, "This instance is already set up.", "Diese Instanz ist bereits eingerichtet.", "Esta instancia ya está configurada."),
        c::UNAUTHORIZED => tr!(l, "Please log in again.", "Bitte melde dich erneut an.", "Vuelve a iniciar sesión."),
        c::FORBIDDEN => tr!(l, "Only admins can do this.", "Das dürfen nur Admins.", "Solo los administradores pueden hacer esto."),
        c::LAST_ADMIN => tr!(l, "You can't remove yourself or the last admin.", "Du kannst dich selbst oder den letzten Admin nicht entfernen.", "No puedes quitarte a ti mismo ni al último administrador."),
        c::NOT_FOUND => tr!(l, "Not found.", "Nicht gefunden.", "No encontrado."),
        "network" => tr!(l, "The server can't be reached.", "Der Server ist nicht erreichbar.", "No se puede conectar con el servidor."),
        _ => tr!(l, "Something went wrong.", "Etwas ist schiefgelaufen.", "Algo salió mal."),
    }
}
