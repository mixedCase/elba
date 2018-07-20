//! Global `elba` configuration file specification.
//!
//! Configuration files change the behavior of the package manager, and can be both global and local
//! to a package. Whereas package files are about specifying a package, config files specify the
//! behavior of the tool itself.
//!
//! Environment variables (.env files?) should also be able to modify the configuration.

use indexmap::IndexMap;

/// The requested verbosity of output
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Verbose,
    Normal,
    Quiet,
}

// TODO: Indices
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub profile: Option<Profile>,
    #[serde(default)]
    pub term: Term,
    #[serde(default = "default_aliases")]
    pub alias: IndexMap<String, String>,
}

impl Config {
    pub fn configure(&mut self, verbosity: Option<Verbosity>, color: Option<bool>) {
        if let Some(v) = verbosity {
            self.term.verbosity = v;
        }
        if let Some(c) = color {
            self.term.color = c;
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            profile: None,
            term: Term::default(),
            alias: default_aliases(),
        }
    }
}

fn default_aliases() -> IndexMap<String, String> {
    indexmap!(
        "i".to_owned() => "install".to_owned(),
        "b".to_owned() => "build".to_owned(),
        "t".to_owned() => "test".to_owned(),
        "r".to_owned() => "run".to_owned(),
    )
}

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct Term {
    pub color: bool,
    pub verbosity: Verbosity,
}

impl Default for Term {
    fn default() -> Self {
        Term {
            color: true,
            verbosity: Verbosity::Normal,
        }
    }
}