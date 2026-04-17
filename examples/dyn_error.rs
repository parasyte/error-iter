use error_iter::ErrorExt as _;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("Failure")]
    Std(#[from] Box<dyn std::error::Error + 'static>),

    #[error("Unknown error")]
    _Unknown,
}

fn main() {
    let error = Box::<dyn std::error::Error + 'static>::from("oh no!");

    let errors = error_strings(&*error);
    assert_eq!(vec!["oh no!"], errors);
    eprintln!("Errors: {errors:#?}");

    let error = Error::from(error);

    let errors = error_strings(&error);
    assert_eq!(vec!["Failure", "oh no!"], errors);
    eprintln!("Errors: {errors:#?}");
}

fn error_strings(err: &(dyn std::error::Error + 'static)) -> Vec<String> {
    #[expect(unstable_name_collisions)]
    err.sources().map(|err| err.to_string()).collect()
}
