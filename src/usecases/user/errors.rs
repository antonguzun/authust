pub enum UserUCError {
    FatalError,
    TemporaryError,
    NotFoundError,
    AlreadyExists,
}

#[derive(Debug)]
pub enum SignError {
    FatalError,
    TemporaryError,
    VerificationError,
}

pub enum AccessModelError {
    FatalError,
    TemporaryError,
    NotFoundError,
    AlreadyExists,
}
