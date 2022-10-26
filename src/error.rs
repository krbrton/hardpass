use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum Error {
    #[error("select data chars, use --help for more information.")]
    SelectCharsError
}
