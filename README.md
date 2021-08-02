# thisdiagnostic

`thisdiagnostic` is a Rust library for adding rich diagnostic metadata to
errors, for some really fancy and customizable error reporting!

## Example

### What You Write

```rust
use thisdiagnostic::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum TaoCompilerError {
    #[diagnostic(
        label = mytool::api::bad_arithmetic
        help = "{num_type} is a number and can only be added to other numbers."
    )]
    #[error("Cannot add types {num_type} and {bad_type}")]
    BadArithmetic {
        num_type: Type,
        bad_type: Type,
        callsite_context: FileContext,
        original_definition: FileContext,
    }

    // ... other errors
}

fn make_addition_err() -> TaoCompilerError {
   TaoCompilerError::BadArithmetic {
       num_type: Type::Nat,
       bad_type: Type::Str,
       callsite_context: FileContext::new("b.tao")
           // Add detail to line 1, columns 11-14, with message
           .detail(1, 11..=14, format!("This is of type {}", Type::Nat))
           .detail(1, 15..=15, format!("{} and {} undergo addition here", Type::Nat, Type::Str))
           .detail(1, 16..=18, format!("This is of type {}", Type::Str)),
       original_definition: FileContext::new("a.tao")
           .detail(1, 5..=8, format!("Original definition of {} here", Var::new("five"))
   }
}

fn main() -> thisdiagnostic::Result<()> {
    thisdiagnostic::reporter::install()?;
    ...
}
```

### What You Get

```ignore
$ ./mytool
Error[mytool::api::bad_arithmetic] Cannot add types Nat and Str
   ╭─[b.tao:1:11]
   │
 1 │ def six = five + "1"
   ·           ──┬─ ┬ ─┬─
   ·             ╰───────── This is of type Nat
   ·                │  │
   ·                │  ╰─── This is of type Str
   ·                │
   ·                ╰──────  Nat and Str undergo addition here
   │
   ├─[a.tao:1:5]
   │
 1 │ def five = 5
   ·     ──┬─
   ·       ╰─── Original definition of five is here
   ·
   · Help: Nat is a number and can only be added to other numbers.
───╯
```

## License

This project and any contributions to it are [licensed under Apache 2.0](LICENSE.md).
