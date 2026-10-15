use super::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AliasStyle {
  Left,
  #[default]
  Right,
  Separate,
}
