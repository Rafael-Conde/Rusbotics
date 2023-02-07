#![warn(clippy::all,
clippy::pedantic,
clippy::perf,
clippy::nursery,
// clippy::cargo,
clippy::unwrap_used,
clippy::expect_used)]

struct symCalculationStateHandler {
    //store an enum with the state machine for the calculations
}

#[cfg(test)]
mod use_case_tests
{
    use std::default::Default;
    use std::fs::read;

    use super::symCalculationStateHandler;

    #[test]
    fn creating_empty_state_handler()
    {
        // should not panic, check for test methodology

        let sym_calculation: symCalculationStateHandler = Default::default();
        assert!(true);
    }

    #[test]
    fn input_data_through_file()
    {
        // should not panic, check for test methodology

        // should also work for xlsx, xls, xlsm, xlsb, xla, xlam
        let file = std::path::Path::new("test_files/test_file1.ods");
        let sym_calculation = symCalculationStateHandler::build_sym_calc().with_data_source(file);
        let other_file = std::path::Path::new("test_files/test_file.ods");
        sym_calculation.reset_to_new_data_source(other_file);
    }

    #[test]
    fn get_dh_matrix_image()
    {
        let file = std::path::Path::new("test_files/test_file1.ods");
        let sym_calculation: symCalculationStateHandler = Default::default();
        sym_calculation.reset_to_new_data_source(file);
        let image = sym_calculation.get_dh_matrix_image().unwrap();

        let reference_image_file = std::path::Path::new("test_files/reference_image.png");
        let reference_image = std::fs::read(reference_image_file).unwrap();
        assert_eq!(image, reference_image);
    }

    #[test]
    fn get_dh_matrix_in_latex_equation()
    {
        use std::fs as fileSystem;
        use std::path::Path;
        let file = Path::new("test_files/test_file1.ods");
        // data source is initially unspecified
        let sym_calculation: symCalculationStateHandler = Default::default();
        sym_calculation.reset_to_new_data_source(file);
        let latex_eq = sym_calculation.get_dh_matrix_latex_eq().unwrap();

        let reference_eq_file = Path::new("test_files/reference_eq.txt");
        let reference_eq = fileSystem::read_to_string(reference_eq_file);
        assert_eq!(referemce_eq, latex_eq);
    }
}
