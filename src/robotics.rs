/// For the Joint Enum the last number inside has a different meaning depending on the Enum
/// variant.
///
/// For the Prismatic variant, the last `f64` means the `Theta` of the joint.
///
/// For the Rotatinal variant, the last `f64` means the `d` variable of the joint.
///
/// For the second number, in both cases it's the value of the `alpha` variable in
/// radians.
use std::fmt::Display;

impl Display for JointType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            JointType::Prismatic(a, rad_alpha, rad_theta) =>
            {
                write!(f,
                       "Prismatic(a: {}, rad_alpha: {}, rad_theta: {})",
                       a, rad_alpha, rad_theta)
            }
            JointType::Rotational(a, rad_alpha, d) =>
            {
                write!(f,
                       "Rotational(a: {}, rad_alpha: {}, d: {})",
                       a, rad_alpha, d)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum JointType
{
    Prismatic(/*a:*/ f64,
              /*rad_alpha:*/ f64,
              /*theta:*/ f64),
    Rotational(/*a:*/ f64, /*rad_alpha:*/ f64, /*d:*/ f64),
}

impl Joint for JointType
{
    fn get_joint_type(&self) -> JointType
    {
        *self
    }
}

pub trait Joint
{
    fn get_joint_type(&self) -> JointType;
}

pub trait DHTable
{
    fn get_joints(&self) -> Vec<Box<dyn Joint>>;
}

pub trait RobotInputData
{
    fn to_dh_table(&self) -> &dyn DHTable;
}

// implementar as funções pros cálculos numéricos
// das equações matemáticas
