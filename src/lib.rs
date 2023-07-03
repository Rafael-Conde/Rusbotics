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

pub mod gui_functions;

pub mod script;

pub mod error
{
    pub mod symbolic_calculation_errors;
}

pub mod robotics;

use crate::robotics::{Errors, Joint, JointType, RIData, RobotInputData};
use calamine::{open_workbook, DataType, Ods, Reader};
use std::{error::Error, path::Path, println}; //, DataType};
use crate::robotics::AttributeType;

// TODO remove all of this things from here

#[cfg(debug_assertions)]
macro_rules! debug_m {
    ($( $args:expr ),*) => { dbg!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(debug_assertions))]
macro_rules! debug_m {
    ($( $args:expr ),*) => {};
}

// implement a function that takes a path as input and returns the dyn RobotInputData trait
// as a way to
fn extract_robot_data_from_file<P: AsRef<Path>>(
    file: P)
    -> Result<Box<dyn RobotInputData>, Box<dyn Error>>
{
    let rows = resolve_path(file)?;
    Ok(Box::new(RIData { vec: process_rows(&rows)? }))
}

fn resolve_path<P>(path: P) -> Result<calamine::Range<DataType>, Box<dyn Error>>
    where P: AsRef<Path>
{
    let path = path.as_ref();
    match path.extension()
              .ok_or("Couldn't retrieve the extension from the path to resolve it")?
              .to_str()
              .ok_or("There was an error with the path")?
    {
        "ods" =>
        {
            #[allow(clippy::option_if_let_else)]
            if let Ok(mut libreoffice) = open_workbook::<Ods<_>, _>(path)
            {
                if let Some(Ok(r)) = libreoffice.worksheet_range_at(0)
                {
                    Ok(r)
                }
                else
                {
                    // std::process::exit(1);
                    Err(Box::new(Errors::SimpleError("Could read the first sheet")))
                }
            }
            else
            {
                // std::process::exit(1);
                Err(Box::new(Errors::SimpleError("Could open the file")))
            }
        }
        _ => Err(Box::new(Errors::SimpleError("File type not supported yet"))),
    }
}

fn process_rows(range: &calamine::Range<DataType>) -> Result<Vec<Box<dyn Joint>>, Box<dyn Error>>
{
    let r_count = range.rows().count();
    if r_count < 1
    {
        debug_m!("substitute print statement for a GUI warning!");

        println!("The number of rows must be at least 1 row composed of 3 numbers and 1 text indicating which column is the joint variable");
        return Err(Box::new(Errors::SimpleError("The file seems to be empty\nThe number of rows must be at least 1 row composed of 3 numbers and 1 text indicating which column is the joint variable")));
    }
    let mut rows = range.rows();
    let mut joints: Vec<Box<dyn Joint>> = Vec::with_capacity(r_count);
    match rows.next()
    {
        Some(&[DataType::String(ref a), DataType::String(ref rad_alpha), DataType::String(ref d), DataType::String(ref theta)]) =>
        {
            if a != "a" || rad_alpha != "rad_alpha" || d != "d" || theta != "rad_theta"
            {
                debug_m!("substitute print statement for a GUI warning!");
                return Err(Box::new(Errors::SimpleError("The first line should be composed only of Strings, the Strings should be the following: \"a\",\"rad_alpha\",\"d\",\"rad_theta\"")));
            }
        }
        // Some(&[DataType::Float(a), DataType::Float(rad_alpha), DataType::Float(d), DataType::String(ref theta)]) =>
        // {
        //     if theta.to_uppercase() == "X"
        //     {
        //         joints.push(Box::new(JointType::Rotational(a, rad_alpha, d)));
        //     }
        // }
        // Some(&[DataType::Float(a), DataType::Float(rad_alpha), DataType::String(ref d), DataType::Float(rad_theta)]) =>
        // {
        //     if d.to_uppercase() == "X"
        //     {
        //         joints.push(Box::new(JointType::Prismatic(a, rad_alpha, rad_theta)));
        //     }
        // }
        None =>
        {
            debug_m!("substitute print statement for a GUI warning!");
            return Err(Box::new(Errors::SimpleError("It seems that the document was empty.")));
        }
        _ =>
        {
            debug_m!("substitute print statement for a GUI warning!");
            return Err(Box::new(Errors::SimpleError("The first line doesn't matche the stablished pattern, checkout the default file to see a template")));
        }
    };
    for row in rows
    {
        match row
        {
            &[ref a @ (DataType::Float(_) | DataType::String(_)), ref rad_alpha @ (DataType::Float(_) | DataType::String(_)), ref d @ (DataType::Float(_) | DataType::String(_)), ref rad_theta @ (DataType::Float(_) | DataType::String(_))] => {
                let joint_a = match *a
                {
                    DataType::String(ref a_symbol) => AttributeType::Symbol(a_symbol.clone()),
                    DataType::Float(a_value) => AttributeType::Value(a_value),
                    _ => unreachable!(),
                };
                let joint_rad_alpha = match *rad_alpha
                {
                    DataType::String(ref rad_alpha_symbol) => AttributeType::Symbol(rad_alpha_symbol.clone()),
                    DataType::Float(rad_alpha_value) => AttributeType::Value(rad_alpha_value),
                    _ => unreachable!(),
                };
                match (d, rad_theta)
                {
                    (DataType::String(ref d_symbol), DataType::String(ref rad_theta_symbol)) => {
                        if (d_symbol == "*") && (rad_theta_symbol == "*")
                        {
                            return Err(Box::new(Errors::SimpleError("There should be only one string \"*\" defining which field is the joint variable, \"d\" or \"rad_theta\"")));
                        }
                        else if (d_symbol != "*") && (rad_theta_symbol != "*")
                        {
                            return Err(Box::new(Errors::SimpleError("There should be at least one string \"*\" defining which field is the joint variable")));
                        }
                        if d_symbol == "*"
                        {
                            let joint_rad_theta = AttributeType::Symbol(rad_theta_symbol.clone());
                            joints.push(Box::new(JointType::Prismatic(joint_a, joint_rad_alpha, joint_rad_theta)));
                        }
                        else if rad_theta_symbol == "*"
                        {
                            let joint_d = AttributeType::Symbol(d_symbol.clone());
                            joints.push(Box::new(JointType::Rotational(joint_a, joint_rad_alpha, joint_d)));
                        }
                        else 
                        {
                            unreachable!();
                        }
                    }
                    (DataType::Float(d_value), DataType::String(ref rad_theta_symbol)) => {
                        println!("{rad_theta_symbol}");
                        if rad_theta_symbol == "*"
                        {
                            let joint_d = AttributeType::Value(*d_value);
                            joints.push(Box::new(JointType::Rotational(joint_a, joint_rad_alpha, joint_d)));
                        }
                        else 
                        {
                            return Err(Box::new(Errors::SimpleError("As \"theta\" is a String, it should be \"*\" indicating it is the joint variable")));
                        }
                    }
                    (DataType::String(ref d_symbol), DataType::Float(rad_theta_value)) => {
                        if d_symbol == "*"
                        {
                            let joint_rad_theta = AttributeType::Value(*rad_theta_value);
                            joints.push(Box::new(JointType::Prismatic(joint_a, joint_rad_alpha, joint_rad_theta)));
                        }
                        else 
                        {
                            return Err(Box::new(Errors::SimpleError("As \"d\" is a String, it should be \"*\" indicating it is the joint variable")));
                        }
                    }
                    (DataType::Float(_), DataType::Float(_)) => {
                        return Err(Box::new(Errors::SimpleError("There should be a \"*\" symbol denoting that that column for that joint a the joint variable")));
                    }
                    (_,_) => return Err(Box::new(Errors::SimpleError("File formating invalid, please check the template file for a reference"))),
                }
            },
            _ =>
            {
                debug_m!("substitute print statement for a GUI warning!");
                return Err(Box::new(Errors::SimpleError("The rows should've 3 numbers and a string, the string should be in either the \"d\" or \"rad_theta\" columns to indicate which one is the joint variable")));
            }
        }
    }
    Ok(joints)
}

// as bad as it's, I'll leave this here in case I quickly need to come back to what it was
// [DataType::Float(a), DataType::Float(rad_alpha), DataType::Float(d), DataType::String(ref theta)] =>
// {
//     if theta.to_uppercase() == "X"
//     {
//         joints.push(Box::new(JointType::Rotational(a, rad_alpha, d)));
//     }
// }
// [DataType::Float(a), DataType::Float(rad_alpha), DataType::String(ref d), DataType::Float(rad_theta)] =>
// {
//     if d.to_uppercase() == "X"
//     {
//         joints.push(Box::new(JointType::Prismatic(a, rad_alpha, rad_theta)));
//     }
// }
