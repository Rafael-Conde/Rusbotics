// ui.label(format!("{:.1}", ctx.input().time));
#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(debug_assertions)]
macro_rules! debug_m {
    ($( $args:expr ),*) => { dbg!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(debug_assertions))]
macro_rules! debug_m {
    ($( $args:expr ),*) => {};
}

use std::{env, u8};
use std::{error::Error, path::Path};

// use Rusbotics::gui_functions::get_gui;
use calamine::{open_workbook, DataType, Ods, Reader}; //, DataType};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use Rusbotics::robotics::{DHTable, Errors, Joint, JointType, RobotInputData};
use Rusbotics::script;

fn main() -> Result<(), Box<dyn Error>>
{
    // let dir = env::current_dir()?.to_string_lossy().to_string();
    // println!("{}", dir);
    // println!("Generating the equation...");
    // let latex = read_to_string("test_file.tex").unwrap();
    //
    // let mut resp: Vec<u8> = tectonic::latex_to_pdf(latex).unwrap();
    // let mut file = OpenOptions::new().write(true)
    //                                  .truncate(true)
    //                                  // either use ? or unwrap since it returns a Result
    //                                  .create(true)
    //                                  .open("test.pdf")?;
    //
    // file.write_all(&mut resp);

    let joints = extract_robot_data_from_file("test_file.ods").unwrap()
                                                              .to_dh_table()
                                                              .get_joints();
    script::get_matrix_image(joints);

    Ok(())
}

// RobotInputData
#[derive(Clone)]
struct RIData
{
    vec: Vec<Box<dyn Joint>>,
}

impl DHTable for RIData
{
    fn get_joints(&self) -> Vec<Box<dyn Joint>>
    {
        self.vec.clone()
    }
}

impl RobotInputData for RIData
{
    fn to_dh_table(&self) -> Box<dyn Rusbotics::robotics::DHTable>
    {
        Box::new(self.clone())
    }
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
    match path.extension().unwrap().to_str().unwrap()
    {
        "ods" =>
        {
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
                return Err(Box::new(Errors::SimpleError("When the first line is composed only of Strings, the Strings should be the following: \"a\",\"rad_alpha\",\"d\",\"rad_theta\"")));
            }
        }
        Some(&[DataType::Float(a), DataType::Float(rad_alpha), DataType::Float(d), DataType::String(ref theta)]) =>
        {
            if theta.to_uppercase() == "X"
            {
                joints.push(Box::new(JointType::Rotational(a, rad_alpha, d)));
            }
        }
        Some(&[DataType::Float(a), DataType::Float(rad_alpha), DataType::String(ref d), DataType::Float(rad_theta)]) =>
        {
            if d.to_uppercase() == "X"
            {
                joints.push(Box::new(JointType::Prismatic(a, rad_alpha, rad_theta)));
            }
        }
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
            &[DataType::Float(a), DataType::Float(rad_alpha), DataType::Float(d), DataType::String(ref theta)] =>
            {
                if theta.to_uppercase() == "X"
                {
                    joints.push(Box::new(JointType::Rotational(a, rad_alpha, d)));
                }
            }
            &[DataType::Float(a), DataType::Float(rad_alpha), DataType::String(ref d), DataType::Float(rad_theta)] =>
            {
                if d.to_uppercase() == "X"
                {
                    joints.push(Box::new(JointType::Prismatic(a, rad_alpha, rad_theta)));
                }
            }
            _ =>
            {
                debug_m!("substitute print statement for a GUI warning!");
                return Err(Box::new(Errors::SimpleError("The rows should've 3 numbers and a string, the string should be in either the \"d\" or \"rad_theta\" columns to indicate which one is the joint variable")));
            }
        }
    }
    Ok(joints)
}
