pub enum Error {
    Input { message: String }
}

pub type Result<T> = std::result::Result<T, Error>;