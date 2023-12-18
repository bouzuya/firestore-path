/// An error that occurs in this crate.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, thiserror::Error)]
pub(crate) enum ErrorKind {
    #[error("collection id conversion {0}")]
    CollectionIdConversion(String),
    #[error("contains invalid charactor")]
    ContainsInvalidCharacter,
    #[error("contains slash")]
    ContainsSlash,
    #[error("document id conversion {0}")]
    DocumentIdConversion(String),
    #[error("ends with hyphen")]
    EndsWithHyphen,
    #[error("invalid name")]
    InvalidName,
    #[error("invalid number of path components")]
    InvalidNumberOfPathComponents,
    #[error("byte length exceeded")]
    LengthOutOfBounds,
    #[error("matches the regular expression `__.*__`")]
    MatchesReservedIdPattern,
    #[error("not contains slash")]
    NotContainsSlash,
    #[error("single period or double periods")]
    SinglePeriodOrDoublePeriods,
    #[error("starts with non letter")]
    StartsWithNonLetter,
}
