use thiserror::{Error};

#[derive(Error, Debug)]
pub enum MiniYamlError {
    #[error("tags are disallowed in minimal-yaml")]
    TagsDisallowed,
    #[error("anchors are disallowed in minimal-yaml")]
    AnchorsDisallowed,
    #[error("aliases are disallowed in minimal-yaml")]
    AliasesDisallowed,
}