use rand::prelude::*;

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

    pub fn hit_points(class: &Class, rng: &mut ThreadRng) -> Result<i8, InputError> {
        match class {
            Class::Warrior => { Ok(rng.gen_range(1..=6) + 10) },
            Class::Wizard => { Ok(rng.gen_range(1..=4) + 8) },
        }
    }
}