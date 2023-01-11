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

use pyo3::{prelude::*, types::PyDict};

use crate::robotics::{Errors, Joint, JointType, RobotInputData};
use std::{env, error::Error};

use pdfium_render::prelude::*;

// implementation of the state machine for the symbolic calculations, so that
// once a step is already calculated, then it isn't necessary to recalculate it
// to get to the next step

struct SymCalculationState
{
    input_data: RIDstate,
}

impl SymCalculationState
{
    fn get_robot_input_data(self, path: AsRef<Path>)
    {
        self.input_data = RIDstate::DataPresent(crate::extract_robot_data_from_file(path));
    }
}

enum RIDstate
{
    NoRobotInputData,
    DataPresent(Box<dyn RobotInputData>),
}

impl Default for RIDstate
{
    fn default() -> Self
    {
        RIDstate::NoRobotInputData
    }
}

// implement a function that converts a Vec<Box<dyn Joint>> into python code that
// can then be converted into the table that is used in the methods for symbolic
// calculatins
fn joints_to_python_code_for_method_input(joints: Vec<Box<dyn Joint>>)
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

pub fn get_matrix_image(joints: Vec<Box<dyn Joint>>) -> Result<(), PyErr>
{
    // add code to read the file instead of including the file String directly
    // env::set_var("FONTCONFIG_PATH", env::current_dir().unwrap());
    let input = joints_to_python_code_for_method_input(joints).unwrap(); // find a way to return
                                                                         // this Err to the caller
    let test_run = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
    let script_library =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/library.py"));
    let mut tex_code = std::string::String::default();
    unsafe {
        pyo3::with_embedded_python_interpreter(|py| -> Result<(), PyErr> {
            let globals = PyDict::new(py);
            let locals = PyDict::new(py);
            PyModule::from_code(py, script_library, "", "library")?;
            py.run(&input, Some(globals), Some(locals))?;
            py.run(test_run, Some(globals), Some(locals))?;
            let latex_equation: &str = locals.get_item("latex_equation").unwrap().extract()?;
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
            Ok(()) // make use of this result
        });
    }

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

    let document = pdfium.load_pdf_from_byte_vec(pdf_bytes, None).unwrap(); // return this Err
                                                                            // to the caller

    let render_config = PdfRenderConfig::new(); // .set_target_width(2000)
                                                // .set_maximum_height(2000);
    for (index, page) in document.pages().iter().enumerate()
    {
        page.render_with_config(&render_config)
            .unwrap() // return this Err to the caller
            .as_image() // Renders this page to an image::DynamicImage...
            .as_rgba8() // ... then converts it to an image::Image...
            .ok_or(PdfiumError::ImageError)
            .unwrap()
            .save_with_format(format!("test-page-{index}.png"), image::ImageFormat::Png) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)
            .unwrap(); // return this Err to the caller
    }
    println!("image generated!");
    Ok(())
}
