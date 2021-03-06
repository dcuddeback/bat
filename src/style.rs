use std::collections::HashSet;
use std::str::FromStr;

use crate::errors::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum StyleComponent {
    Auto,
    Changes,
    Grid,
    Header,
    Numbers,
    Snip,
    Full,
    Plain,
}

impl StyleComponent {
    pub fn components(self, interactive_terminal: bool) -> &'static [StyleComponent] {
        match self {
            StyleComponent::Auto => {
                if interactive_terminal {
                    StyleComponent::Full.components(interactive_terminal)
                } else {
                    StyleComponent::Plain.components(interactive_terminal)
                }
            }
            StyleComponent::Changes => &[StyleComponent::Changes],
            StyleComponent::Grid => &[StyleComponent::Grid],
            StyleComponent::Header => &[StyleComponent::Header],
            StyleComponent::Numbers => &[StyleComponent::Numbers],
            StyleComponent::Snip => &[StyleComponent::Snip],
            StyleComponent::Full => &[
                StyleComponent::Changes,
                StyleComponent::Grid,
                StyleComponent::Header,
                StyleComponent::Numbers,
                StyleComponent::Snip,
            ],
            StyleComponent::Plain => &[],
        }
    }
}

impl FromStr for StyleComponent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "auto" => Ok(StyleComponent::Auto),
            "changes" => Ok(StyleComponent::Changes),
            "grid" => Ok(StyleComponent::Grid),
            "header" => Ok(StyleComponent::Header),
            "numbers" => Ok(StyleComponent::Numbers),
            "snip" => Ok(StyleComponent::Snip),
            "full" => Ok(StyleComponent::Full),
            "plain" => Ok(StyleComponent::Plain),
            _ => Err(format!("Unknown style '{}'", s).into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StyleComponents(pub HashSet<StyleComponent>);

impl StyleComponents {
    pub fn new(components: &[StyleComponent]) -> StyleComponents {
        StyleComponents(components.iter().cloned().collect())
    }

    #[cfg(feature = "git")]
    pub fn changes(&self) -> bool {
        self.0.contains(&StyleComponent::Changes)
    }

    pub fn grid(&self) -> bool {
        self.0.contains(&StyleComponent::Grid)
    }

    pub fn header(&self) -> bool {
        self.0.contains(&StyleComponent::Header)
    }

    pub fn numbers(&self) -> bool {
        self.0.contains(&StyleComponent::Numbers)
    }

    pub fn snip(&self) -> bool {
        self.0.contains(&StyleComponent::Snip)
    }

    pub fn plain(&self) -> bool {
        self.0.iter().all(|c| c == &StyleComponent::Plain)
    }
}
