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

use std::{error::Error, fmt::Display};

impl Display for JointType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Prismatic(a, rad_alpha, rad_theta) =>
            {
                write!(f,
                       "Prismatic(a: {a}, rad_alpha: {rad_alpha}, rad_theta: {rad_theta})")
            }
            Self::Rotational(a, rad_alpha, d) =>
            {
                write!(f, "Rotational(a: {a}, rad_alpha: {rad_alpha}, d: {d})")
            }
        }
    }
}

/// For the Joint Enum the last number inside has a different meaning depending on the Enum
/// variant.
///
/// For the Prismatic variant, the last `f64` means the `Theta` of the joint.
///
/// For the Rotatinal variant, the last `f64` means the `d` variable of the joint.
///
/// For the second number, in both cases it's the value of the `alpha` variable in
/// radians.
#[derive(Copy, Clone, Debug)]
pub enum JointType
{
    Prismatic(f64, f64, f64),
    Rotational(f64, f64, f64),
}

impl Joint for JointType
{
    fn get_joint_type(&self) -> JointType
    {
        *self
    }
}

pub trait Joint: private_parts::JointClone
{
    fn get_joint_type(&self) -> JointType;
}

impl<T> private_parts::JointClone for T where T: 'static + Joint + Clone
{
    fn joint_clone(&self) -> Box<dyn Joint>
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Joint>
{
    fn clone(&self) -> Self
    {
        self.joint_clone()
    }
}

mod private_parts
{
    pub trait JointClone
    {
        fn joint_clone(&self) -> Box<dyn super::Joint>;
    }
}

pub trait DHTable
{
    fn get_joints(&self) -> Vec<Box<dyn Joint>>;
}

pub trait RobotInputData
{
    fn to_dh_table(&self) -> Box<dyn DHTable>;
}

// implementar as funções pros cálculos numéricos
// das equações matemáticas

// lugar temporário para essa enum
#[derive(Debug, Copy, Clone)]
pub enum Errors
{
    SimpleError(&'static str),
}

impl Error for Errors {} // no implementation necessary, since I'll be only using format
                         // and debug traits

impl Display for Errors
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        #[allow(clippy::infallible_destructuring_match)]
        let msg = match self
        {
            Self::SimpleError(msg) => msg,
        };
        write!(f, "{msg}")
    }
}

// RobotInputData
#[derive(Clone)]
pub(crate) struct RIData
{
    pub(crate) vec: Vec<Box<dyn Joint>>,
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
    fn to_dh_table(&self) -> Box<dyn DHTable>
    {
        Box::new(self.clone())
    }
}
