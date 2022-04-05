#[derive(Debug)]
pub enum Class {
    Warrior,
    Wizard,
}

#[derive(Debug)]
pub struct InputError;

impl Class {
    pub fn from_str(s: &str) -> Result<Class, InputError> {
        match s {
            "warrior" => Ok(Class::Warrior),
            "wizard" => Ok(Class::Wizard),
            _ => Err(InputError),
        }
    }
}