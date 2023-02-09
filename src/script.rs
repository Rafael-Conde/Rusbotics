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

use pyo3::prelude::*;
use pyo3::types::PyDict;

pub mod SymCalculationState;

use crate::robotics::{Errors, Joint, JointType};
use std::{env, error::Error, u8, io::Cursor};
use std::convert::AsRef;
use std::iter::Iterator;
use std::iter::IntoIterator;

use pdfium_render::prelude::*;


// implement a function that converts a Vec<Box<dyn Joint>> into python code that
// can then be converted into the table that is used in the methods for symbolic
// calculatins
fn joints_to_python_code_for_method_input<'a, I>(joints: I)
                                          -> Result<String, Box<dyn Error>>
where I: ExactSizeIterator<Item = &'a Box<dyn Joint>> + Clone
{
    // let joints = joints.peekable();
    if joints.len() == 0
    {
        return Err(Box::new(Errors::SimpleError("No Joint was provided")));
    }
    let mut python_code_input = String::from("tabela_DH = ([[");
    let mut tailing = "],[";
    // let mut joint_peekable = joints.clone().peekable();
    let mut joint_peekable = joints.clone().peekable();
    let mut i = 0;
    while let Some(joint) = joint_peekable.next()
    {
        i += 1;
        if joint_peekable.peek().is_none()
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
// get_dh_matrix_image_eq_and_py functron
pub fn get_dh_matrix_image<'a, C>(joints: C) -> Result<(Vec<u8>,String,Py<PyAny>), Box<dyn Error>>
where C: AsRef<[Box<Joint>]>
{
    let input = joints_to_python_code_for_method_input(joints.as_ref().into_iter())?;
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
