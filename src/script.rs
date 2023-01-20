// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
// Copyright (c) 2023 Rafael de Conde Reis. All rights reserved.

#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]
// #![allow(clippy::unwrap_used)]

use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};

use crate::robotics::{Errors, Joint, JointType, RobotInputData};
use std::{env, error::Error, path::Path, u8, io::Cursor};

use pdfium_render::prelude::*;

// implementation of the state machine for the symbolic calculations, so that
// once a step is already calculated, then it isn't necessary to recalculate it
// to get to the next step

struct SymCalculation
{
    state: SymCalculationState,
}

impl SymCalculation
{
    // implement a separated method to get the input data from a string
    pub fn set_robot_input_data<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    {
        self.state = SymCalculationState::HaveRobotData(crate::extract_robot_data_from_file(path)?);
        Ok(())
    }

    pub fn get_joints(&mut self) {}
}

#[derive(Default)]
enum SymCalculationState
{
    #[default]
    NotStarted,
    HaveRobotData(Box<dyn RobotInputData>),
    DHMatrixCalculated
    {
        python_list_of_matrices: Py<PyList>,
        matrix_image: Vec<u8>,
        eq_tex: String,
    },
}

// implement a function that converts a Vec<Box<dyn Joint>> into python code that
// can then be converted into the table that is used in the methods for symbolic
// calculatins
fn joints_to_python_code_for_method_input(joints: &Vec<Box<dyn Joint>>)
                                          -> Result<String, Box<dyn Error>>
{
    if joints.is_empty()
    {
        return Err(Box::new(Errors::SimpleError("No Joint was provided")));
    }
    let mut python_code_input = String::from("tabela_DH = ([[");
    let mut tailing = "],[";
    let mut joint_it = joints.iter().peekable();
    let mut i = 0;
    while let Some(joint) = joint_it.next()
    {
        i += 1;
        if joint_it.peek().is_none()
        {
            tailing = "]])\n\n";
        }
        match joint.get_joint_type()
        {
            JointType::Prismatic(a, rad_alpha, theta) =>
            {
                python_code_input.push_str(&format!("'{rad_alpha}','{a}','d_{i}','{theta}'{tailing}"));
            }
            JointType::Rotational(a, rad_alpha, d) =>
            {
                python_code_input.push_str(&format!("'{rad_alpha}','{a}','{d}','theta_{i}'{tailing}"));
            }
        }
    }
    tailing = "','";
    python_code_input.push_str("joints = (['");
    let mut joint_it = joints.into_iter().peekable();
    while let Some(joint) = joint_it.next()
    {
        if joint_it.peek().is_none()
        {
            tailing = "'])\n\n";
        }
        match joint.get_joint_type()
        {
            JointType::Prismatic(_, _, _) =>
            {
                python_code_input.push('p');
                python_code_input.push_str(tailing);
            }
            JointType::Rotational(_, _, _) =>
            {
                python_code_input.push('r');
                python_code_input.push_str(tailing);
            }
        }
    }
    Ok(python_code_input)
}

pub fn get_matrix_image(joints: &Vec<Box<dyn Joint>>) -> Result<(Vec<u8>,String,Py<PyAny>), Box<dyn Error>>
{
    let input = joints_to_python_code_for_method_input(joints)?;
    let test_run = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
    let script_library =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/library.py"));
    let mut tex_code = std::string::String::default();
    let (dh_list, latex_equation) = unsafe {
        pyo3::with_embedded_python_interpreter(|py| -> Result<(Py<PyAny>,String), Box<dyn Error>> {
            let globals = PyDict::new(py);
            let locals = PyDict::new(py);
            PyModule::import(py, "sympy")?;
            PyModule::from_code(py, script_library, "", "library")?;
            py.run(&input, Some(globals), Some(locals))?;
            py.run(test_run, Some(globals), Some(locals))?;
            let error = Errors::SimpleError("Error getting DH matrix symbolically");
            let dh_list: Py<PyAny> = locals.get_item("matrix_dh")
                .ok_or(error)?
                .into();

            let latex_equation: &str = locals.get_item("latex_equation")
                .ok_or(Errors::SimpleError("Error getting latex equation"))?
                .extract()?;

            let latex_equation = latex_equation.to_string();
            tex_code = format!(
                "
                \\documentclass{{standalone}}
                \\usepackage{{amsmath}}
                \\begin{{document}}
	\\( \\displaystyle {latex_equation} \\)
    \\end{{document}}
"
);
            Ok((dh_list,latex_equation)) 
        })?
    };

    println!("The text is: \n{tex_code}");
    let pdf_bytes: Vec<u8> = tectonic::latex_to_pdf(tex_code).unwrap(); // find a way
                                                                        // to return this
                                                                        // Err to the caller
                                                                        // let mut file = OpenOptions::new().write(true)
                                                                        //                                  .truncate(true)
                                                                        //                                  .create(true)
                                                                        //                                  .open("test.pdf")?;
                                                                        //
                                                                        // file.write_all(&mut resp);
    let pdfium = Pdfium::new(
	    Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
	    .or_else(|_| Pdfium::bind_to_system_library()).unwrap()); // show a pop-up warning
                                                                  // and close the program

                                                                  // let document = pdfium.load_pdf_from_bytes(pdf_bytes., None).unwrap();

    let document =
        pdfium.load_pdf_from_byte_vec(pdf_bytes, None)
        .map_err(|_err| Errors::SimpleError("Error while processing the image's data"))?;

    let render_config = PdfRenderConfig::new().set_target_width(3000)
                                              .set_maximum_height(4000);
    let mut image_bytes = Vec::new();
    let page = document.pages()
        .first()
        .map_err(|_err| Errors::SimpleError("Empty image generated"))?;
    page.render_with_config(&render_config)
        .map_err(|_err| Errors::SimpleError("Error while rendering generated image"))?
        .as_image()
        .write_to(&mut Cursor::new(&mut image_bytes), image::ImageOutputFormat::Png)
        .map_err(|_err| Box::new(Errors::SimpleError("Error while getting the bytes for the image")))?;
    println!("image generated!");
    Ok((image_bytes, latex_equation, dh_list)) 
}
