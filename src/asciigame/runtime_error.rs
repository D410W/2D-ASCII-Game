pub enum RuntimeError {
  TryFromIntError {
    source: std::num::TryFromIntError,
    context: String,
  },
}

impl RuntimeError {
  pub fn new(p_err: std::num::TryFromIntError, p_context: &str) -> RuntimeError {
    return RuntimeError::TryFromIntError{ source: p_err, context: p_context.to_string() };
  }
}
