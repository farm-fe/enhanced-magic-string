#[derive(Debug)]
pub enum Error {
  IllegalSource,
}

pub type Result<T> = std::result::Result<T, Error>;
