pub enum UserUCError {
    FatalError,
    TemporaryError,
    NotFoundError,
    AlreadyExists,
}

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
