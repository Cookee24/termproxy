use std::borrow::Cow;
use std::str::FromStr;

use clap::Parser;
use colored::Colorize;

pub type Var<'a> = (&'static str, Cow<'a, str>);
pub type Vars<'a> = Vec<Var<'a>>;

#[derive(Parser, Debug, Clone, Copy, PartialEq)]
pub enum Terminal {
    Bash,
    Cmd,
    Elvish,
    Fish,
    Ion,
    Nu,
    PowerShell,
    Tcsh,
    Xonsh,
    Zsh,
}

impl Terminal {
    const fn valid_terminals() -> &'static str {
        "bash, cmd, elvish, fish, ion, nu, powershell, tcsh, xonsh, zsh"
    }
}

impl FromStr for Terminal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Terminal::Bash),
            "cmd" => Ok(Terminal::Cmd),
            "elvish" => Ok(Terminal::Elvish),
            "fish" => Ok(Terminal::Fish),
            "ion" => Ok(Terminal::Ion),
            "nu" => Ok(Terminal::Nu),
            "powershell" => Ok(Terminal::PowerShell),
            "tcsh" => Ok(Terminal::Tcsh),
            "xonsh" => Ok(Terminal::Xonsh),
            "zsh" => Ok(Terminal::Zsh),
            _ => Err(format!(
                "Invalid terminal, valid terminals are: {}",
                Terminal::valid_terminals().bold()
            )),
        }
    }
}

impl Terminal {
    #[inline]
    fn escape_value<'a>(&self, value: Cow<'a, str>) -> Cow<'a, str> {
        match self {
            Terminal::PowerShell => {
                if value.contains('`') || value.contains('"') {
                    Cow::Owned(value.replace("`", "``").replace("\"", "`\""))
                } else {
                    value
                }
            }
            Terminal::Cmd => {
                if value.contains('%') {
                    Cow::Owned(value.replace("%", "%%"))
                } else {
                    value
                }
            }
            Terminal::Bash | Terminal::Zsh | Terminal::Fish | Terminal::Elvish | Terminal::Xonsh | Terminal::Tcsh | Terminal::Ion | Terminal::Nu => {
                if value.contains('\\') || value.contains('"') {
                    Cow::Owned(value.replace("\\", "\\\\").replace("\"", "\\\""))
                } else {
                    value
                }
            }
        }
    }

    #[inline]
    pub fn set_env_str<'a>(&self, (key, value): Var<'a>) -> String {
        let escaped_value = self.escape_value(value);
        match self {
            Terminal::PowerShell => format!("$env:{key} = \"{escaped_value}\""),
            Terminal::Cmd => format!("set {key}={escaped_value}"),
            Terminal::Bash => format!("export {key}=\"{escaped_value}\""),
            Terminal::Zsh => format!("export {key}=\"{escaped_value}\""),
            Terminal::Fish => format!("set -x {key} \"{escaped_value}\""),
            Terminal::Elvish => format!("set-env {key} \"{escaped_value}\""),
            Terminal::Xonsh => format!("$env:{key} = \"{escaped_value}\""),
            Terminal::Tcsh => format!("setenv {key} \"{escaped_value}\""),
            Terminal::Ion => format!("export {key}=\"{escaped_value}\""),
            Terminal::Nu => format!("$nu.env[\"{key}\"] = \"{escaped_value}\""),
        }
    }

    pub fn set_envs_str<'a>(&self, envs: Vars<'a>) -> String {
        envs.into_iter()
            .map(|(key, value)| self.set_env_str((key, value)))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
