#![warn(clippy::all,
clippy::pedantic,
clippy::perf,
clippy::nursery,
// clippy::cargo,
clippy::unwrap_used,
clippy::expect_used)]

use image::DynamicImage;
use pyo3::{Py, PyAny};

use crate::{
    error::symbolic_calculation_errors::SymbolicCalculationError,
    robotics::{Joint, JointType},
};

pub struct SymCalculationStateHandler
{
    data_source: DataSource,
    joints: Vec<JointType>,
    symbolic_values: SymbolicValuesStorage,
}

//build with builder pattern
impl SymCalculationStateHandler
{
    pub fn reset_to_new_path_source<P: AsRef<Path>>(self, path: P)
                                                    -> Result<(), std::io::ErrorKind>
    {
    }

    pub fn reset_to_new_csv_source<S: AsRef<&str>>(self, csv_data: S) {}

    pub fn get_dh_matrix_image<J>(joints_slice: J) -> Result<DynamicImage, image::error::ImageError>
        where J: AsRef<[Joints]>
    {
    }

    pub fn get_dh_matrix_png_image_bytes<J>(joints_slice: J)
                                            -> Result<Vec<u8>, image::error::ImageError>
        where J: AsRef<[Joints]>
    {
    }

    pub fn get_dh_matrix_latex_eq<J>(joints_slice: J) -> Result<String, SymbolicCalculationError>
        where J: AsRef<[Joints]>
    {
    }
}

struct SymbolicValuesStorage
{
    dh_matrix_symbolic: DHMatrixSymbolic,
    jacobian_symbolic: JacobianSymbolic,
}

struct InnerValues
{
    python_value: Option<Py<PyAny>>,
    image: Option<DynamicImage>,
    latex_eq: Option<String>,
}

struct SymbolicDHMatrixStorage
{
    inner: InnerValues,
}

impl SymbolicDHMatrixStorage
{
    fn reset(&mut self) {}

    fn get_image<J>(&self) -> Option<DynamicImage> {}

    fn get_ref_image<J>(&self) -> Option<&DynamicImage> {}

    fn store_image<J>(&self, dyn_image: DynamicImage) {}

    fn store_latex_eq<J>(&self, latex_eq: String) {}

    fn get_latex_eq<J>(&self) -> Option<String> {}

    fn get_ref_latex_eq<J>(&self) -> Option<&str> {}

    fn store_python_value(&mut self, py_value: Py<PyAny>) {}

    fn get_python_value(&mut self) -> Result<Py<PyAny>, SymbolicCalculationError> {}
}

struct SymbolicJacobianStorage
{
    inner: InnerValues,
}

struct SymCalculationStateHandlerBuilder;

impl SymCalculationStateHandlerBuilder
{
    pub fn builder() -> SymCalculationStateHandlerBuilder
    {
        SymCalculationStateHandlerBuilder
    }

    pub fn build(self) -> SymCalculationStateHandler {}

    pub fn with_path_data_source<DS>(self, path: DS)
        where DS: AsRef<Path>
    {
        todo!()
    }

    pub fn with_csv_data_source<DS>(self, csv_data: DS)
        where DS: AsRef<str>
    {
        todo!()
    }
}

#[cfg(test)]
mod use_case_tests
{
    use std::default::Default;
    use std::fs::read;

    fn generate_vec_of_joints() -> Vec<JointType>
    {
        vec![JointType::Rotational(1, 90, 100),
             JointType::Prismatic(2, 180, 200)]
    }

    use super::SymCalculationStateHandler;
    mod sym_state_handler
    {
        use std::io::{BufWriter, Cursor};

        use image;
        #[test]
        fn creating_empty_state_handler()
        {
            let sym_calculation: SymCalculationStateHandler = Default::default();
        }

        #[test]
        fn input_data_through_file()
        {
            // should also work for xlsx, xls, xlsm, xlsb, xla, xlam
            let file = std::path::Path::new("test_files/test_file.ods");
            let sym_calculation =
                SymCalculationStateHandler::build_sym_calc().with_data_source(file);
            let other_file = std::path::Path::new("test_files/test_file.ods");
            sym_calculation.reset_to_new_data_source(other_file);
        }

        #[test]
        fn get_dh_matrix_image()
        {
            let file = std::path::Path::new("test_files/test_file.ods");
            let sym_calculation: SymCalculationStateHandler = Default::default();
            sym_calculation.reset_to_new_data_source(file);
            let image = sym_calculation.get_dh_matrix_image().unwrap();

            let reference_image_file = std::path::Path::new("test_files/reference_image.png");
            let reference_image = std::fs::read(reference_image_file).unwrap();

            // necessary to compare the bytes of the images
            let image_bytes = Vec::new_with_capacity(image.width() * image.height());
            image.write_to(BufWriter::new(&mut image_bytes),
                           image::ImageOutputFormat::Png);
            assert_eq!(image_bytes, reference_image);
        }

        #[test]
        fn get_dh_matrix_in_latex_equation()
        {
            use std::fs as fileSystem;
            use std::path::Path;
            let file = Path::new("test_files/test_file.ods");
            // data source is initially unspecified
            let sym_calculation: SymCalculationStateHandler = Default::default();
            sym_calculation.reset_to_new_data_source(file);
            let latex_eq = sym_calculation.get_dh_matrix_latex_eq().unwrap();

            let reference_eq_file = Path::new("test_files/reference_eq.txt");
            let reference_eq = fileSystem::read_to_string(reference_eq_file);
            assert_eq!(referemce_eq, latex_eq);
        }

        #[test]
        fn get_ref_dh_matrix_image()
        {
            let file = std::path::Path::new("test_files/test_file.ods");
            let sym_calculation: SymCalculationStateHandler = Default::default();
            sym_calculation.reset_to_new_data_source(file);
            let image = sym_calculation.get_ref_dh_matrix_image().unwrap();

            let reference_image_file = std::path::Path::new("test_files/reference_image.png");
            let reference_image = std::fs::read(reference_image_file).unwrap();

            // necessary to compare the bytes of the images
            let image_bytes = Vec::new_with_capacity(image.width() * image.height());
            image.write_to(BufWriter::new(&mut image_bytes),
                           image::ImageOutputFormat::Png);
            assert_eq!(image_bytes, reference_image);
        }

        #[test]
        fn get_ref_dh_matrix_in_latex_equation()
        {
            use std::fs as fileSystem;
            use std::path::Path;
            let file = Path::new("test_files/test_file.ods");
            // data source is initially unspecified
            let sym_calculation: SymCalculationStateHandler = Default::default();
            sym_calculation.reset_to_new_data_source(file);
            let latex_eq = sym_calculation.get_ref_dh_matrix_latex_eq().unwrap();

            let reference_eq_file = Path::new("test_files/reference_eq.txt");
            let reference_eq = fileSystem::read_to_string(reference_eq_file);
            assert_eq!(referemce_eq, *latex_eq);
        }
    }

    mod functions_use_cases
    {
        use crate::robotics::JointType;

        #[test]
        fn function_dh_matrix_image_from_joints()
        {
            let joints = generate_vec_of_joints();
            let image = generate_dh_matrix_image(&joints).unwrap();

            let reference_image_file = std::path::Path::new("test_files/reference_image.png");
            let reference_image = std::fs::read(reference_image_file).unwrap();

            // necessary to compare the bytes of the images
            let image_bytes = Vec::new_with_capacity(image.width() * image.height());
            image.write_to(BufWriter::new(&mut image_bytes),
                           image::ImageOutputFormat::Png);
            assert_eq!(image_bytes, reference_image);
        }

        #[test]
        fn function_dh_matrix_image_bytes_from_joints()
        {
            let joints = generate_vec_of_joints();
            let image = generate_dh_matrix_image(&joints).unwrap();

            let reference_image_file = std::path::Path::new("test_files/reference_image.png");
            let reference_image = std::fs::read(reference_image_file).unwrap();

            // necessary to compare the bytes of the images
            let image_bytes = Vec::new_with_capacity(image.width() * image.height());
            image.write_to(BufWriter::new(&mut image_bytes),
                           image::ImageOutputFormat::Png);
            assert_eq!(image_bytes, reference_image);
        }

        #[test]
        fn function_dh_matrix_in_latex_equation()
        {
            use std::fs as fileSystem;
            use std::path::Path;
            let joints = generate_vec_of_joints();
            let latex_eq = generate_dh_matrix_latex_eq(&joints).unwrap();

            let reference_eq_file = Path::new("test_files/reference_eq.txt");
            let reference_eq = fileSystem::read_to_string(reference_eq_file);
            assert_eq!(referemce_eq, latex_eq);
        }
    }

    mod SymbolicValuesStorage_test
    {
        use crate::script::SymCalculationState::SymbolicValuesStorage;

        use super::generate_vec_of_joints;

        #[test]
        fn symbolic_values_storage_creation()
        {
            let symbolic_values: SymbolicValuesStorage = Default::default();
        }

        #[test]
        fn initialization()
        {
            let symbolic_values = SymbolicValuesStorage::new(generate_vec_of_joints());
        }

        #[test]
        fn storing_dh_matrix_image()
        {
            let symbolic_values: SymbolicValuesStorage = Default::default();
            let image = generate_dh_matrix_image(generate_vec_of_joints());
            symbolic_values.store_image(image);
            assert_eq!(image, *(symbolic_values.get_ref_image()));
        }

        #[test]
        fn storing_dh_matrix_latex_eq()
        {
            let symbolic_values: SymbolicValuesStorage = Default::default();
            let latex_eq = generate_dh_matrix_latex_eq(generate_vec_of_joints());
            symbolic_values.store_latex_eq(latex_eq);
            assert_eq!(latex_eq, *(symbolic_values.get_ref_latex_eq()));
        }

        fn storing_jacobian_image()
        {
            let symbolic_values: SymbolicValuesStorage = Default::default();
            let image = generate_jacobian_image(generate_vec_of_joints());
            symbolic_values.store_image(image);
            assert_eq!(image, *(symbolic_values.get_ref_image()));
        }

        #[test]
        fn storing_jacobian_latex_eq()
        {
            let symbolic_values: SymbolicValuesStorage = Default::default();
            let latex_eq = generate_jacobian_latex_eq(generate_vec_of_joints());
            symbolic_values.store_latex_eq(latex_eq);
            assert_eq!(latex_eq, *(symbolic_values.get_ref_latex_eq()));
        }
    }
}
