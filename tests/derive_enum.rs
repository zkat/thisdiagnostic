use thisdiagnostic::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Rainbow error.")]
#[label("critical::rainbow")]
#[help("Rainbow.")]
pub struct Rainbow;

#[derive(Debug, Error, Diagnostic)]
#[error("Critical error.")]
pub enum Critical {
    #[label("critical::blue")]
    #[help("Blue.")]
    Blue(String),
    #[label("critical::red")]
    #[help("Red.")]
    Red,
    #[label("critical::orange")]
    #[help("Orange.")]
    Orange,
    Transparent(#[ask] Rainbow),
}

#[test]
fn it_works() {
    let blue = Critical::Blue("test".into());
    assert_eq!("Blue.", blue.help().unwrap());
    assert_eq!("critical::blue", blue.label());

    let red = Critical::Red;
    assert_eq!("Red.", red.help().unwrap());
    assert_eq!("critical::red", red.label());

    let orange = Critical::Orange;
    assert_eq!("Orange.", orange.help().unwrap());
    assert_eq!("critical::orange", orange.label());

    let rainbow = Rainbow {};

    let transp = Critical::Transparent(rainbow);
    assert_eq!("Rainbow.", transp.help().unwrap());
    assert_eq!("critical::rainbow", transp.label());
}
