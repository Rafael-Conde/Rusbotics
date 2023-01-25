// use crate::RobotInputData;
// use pyo3::{Py, PyAny};
// use std::error::Error;
// use std::path::Path;
//
// // implementation of the state machine for the symbolic calculations, so that
// // once a step is already calculated, then it isn't necessary to recalculate it
// // to get to the next step
//
// struct SymCalculation
// {
//     state: SymCalculationStates,
// }
//
// impl SymCalculation
// {
//     // implement a separated method to get the input data from a string
//     pub fn set_robot_input_data<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>>
//     {
//         self.state =
//             SymCalculationStates::HaveRobotData(crate::extract_robot_data_from_file(path)?);
//         Ok(())
//     }
//
//     pub fn get_joints(&mut self) {}
// }
//
// struct SymCalculationState<RobotData, Joints, PyState> {}
//
// #[derive(Default)]
// enum SymCalculationStates
// {
//     #[default]
//     NotStarted,
//     HaveRobotData(Box<dyn RobotInputData>),
//     DHMatrixCalculated
//     {
//         python_list_of_matrices: Py<PyAny>,
//         matrix_image: Vec<u8>,
//         eq_tex: String,
//     },
// }
