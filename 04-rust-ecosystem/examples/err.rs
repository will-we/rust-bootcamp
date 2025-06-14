use rust_ecosystem::MyError;

fn main() -> Result<(), MyError> {
    Ok(fail_with_error()?)
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::InvalidArgument(
        "Name Invalid argument".to_string(),
    ))
}
