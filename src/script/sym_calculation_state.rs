#![warn(clippy::all,
clippy::pedantic,
clippy::perf,
clippy::nursery,
// clippy::cargo,
clippy::unwrap_used,
clippy::expect_used)]

use crate::robotics::Errors;
use csv;
use serde::Deserialize;
use std::error::Error;
use std::str::FromStr;

#[derive(Deserialize)]
enum ExprInput
{
    Symbolic(String),
    Numeric(f64),
}

#[derive(Deserialize)]
struct DHTableRow
{
    a: ExprInput,
    rad_alpha: ExprInput,
    d: ExprInput,
    rad_theta: ExprInput,
}

struct SymState {}

enum InputKind
{
    File,
    Csv,
}

impl SymState
{
    pub fn new<'a>(input: &'a str) -> Self
    {
        SymState {}
    }

    fn kind_of_input<'a>(input: &'a str) -> Result<InputKind, Errors>
    {
        if let Ok(path) = std::path::PathBuf::from_str(input)
        {
            return Ok(InputKind::File);
        }
        let mut reader = csv::Reader::from_reader(input.as_bytes());
        let mut dhtable: Vec<DHTableRow> = Vec::new();
        for row in reader.deserialize()
        {
            let row: DHTableRow = row.map_err(|_e| Errors::SimpleError("Invalid Input"))?;
            dhtable.push(row);
        }
        Err(Errors::SimpleError("Invalid Input"))
    }
}

#[cfg(test)]
mod use_case_tests
{
    use super::InputKind;
    use super::SymState;
    #[test]
    fn creating_symbolic_calculation_state()
    {
        let sym_state = SymState::new("test_files/test_file1.ods");
        let sym_state2 = SymState::new("a rad_alpha d rad_theta\n1 90 100 X\n2 180 200 X");
    }

    #[test]
    fn identifying_path_input()
    {
        let sym_state = SymState::kind_of_input("test_files/test_file1.ods");
        assert!(match sym_state
                {
                    Ok(InputKind::File) => true,
                    _ => false,
                });
    }

    #[test]
    fn identifying_csv_input()
    {
        let sym_state2 =
            SymState::kind_of_input("a rad_alpha d rad_theta\n1 90 100 X\n2 180 200 X");
        assert!(match sym_state2
                {
                    Ok(InputKind::Csv) => true,
                    _ => false,
                })
    }
}

