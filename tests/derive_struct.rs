use thisdiagnostic::Diagnostic;
use thiserror::Error;

#[derive(Diagnostic, Debug, Eq, PartialEq, Error)]
#[error("Colored struct.")]
#[label("color::struct")]
#[help("Color.")]
pub struct Color {
    input: Option<String>,
    field: i32,
}

#[test]
fn it_works() {
    let clr = Color {
        field: 1,
        input: Some("lol".into()),
    };
    assert_eq!("color::struct", clr.label());
    assert_eq!("Color.", clr.help().unwrap());
}
