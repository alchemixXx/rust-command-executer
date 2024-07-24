pub type CustomResult<T> = core::result::Result<T, CustomError>;

#[derive(Debug)]
pub enum CustomError {
    CommandExecution,
    PathBuildException,
}

impl std::error::Error for CustomError {}
impl core::fmt::Display for CustomError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}
