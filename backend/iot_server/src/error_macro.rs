#[macro_export]
macro_rules! generate_internal_server_error {
  ($err:ty) => {
    impl From<$err> for APIError {
      fn from(_: $err) -> Self {
        APIError::InternalServerError
      }
    }
  };
}
