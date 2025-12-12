#[allow(dead_code)]

#[derive(Debug)]
pub enum RuntimeError {
  GenericError {
    context: String,
  },
  TryFromIntError {
    source: std::num::TryFromIntError,
    context: String,
  },
  IoError {
    source: std::io::Error,
    context: String,
  },
  SystemTimeError {
    source: std::time::SystemTimeError,
    context: String,    
  },
}

impl RuntimeError {

  #[allow(non_snake_case)]
  pub fn TryFromIntError(p_err: std::num::TryFromIntError, p_context: &str) -> Self {
    return RuntimeError::TryFromIntError{ source: p_err, context: p_context.to_string() };
  }
  
  // pub fn new(p_err: std::io::Error, p_context : &str) -> Self {
  //   return RuntimeError::IoError{ source: p_err, context: p_context.to_string() };
  // }

}

impl std::convert::From<std::io::Error> for RuntimeError {
  fn from(p_err: std::io::Error) -> Self {
    return RuntimeError::IoError{ source: p_err, context: String::new() };
  }
}

impl std::convert::From<std::num::TryFromIntError> for RuntimeError {
  fn from(p_err: std::num::TryFromIntError) -> Self {
    return RuntimeError::TryFromIntError{ source: p_err, context: String::new() };
  }
}

impl std::convert::From<std::time::SystemTimeError> for RuntimeError {
  fn from(p_err: std::time::SystemTimeError) -> Self {
    return RuntimeError::SystemTimeError{ source: p_err, context: String::new() };
  }
}
