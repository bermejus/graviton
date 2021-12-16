use std::collections::HashMap;
use crate::celestial_body::*;

#[derive(Debug)]
pub enum ParamEnum<'a> {
    I32(i32),
    F64(f64),
    Bodies(Bodies<'a>)
}

impl From<i32> for ParamEnum<'_> {
    fn from(v: i32) -> Self { ParamEnum::I32(v) }
}

impl From<f64> for ParamEnum<'_> {
    fn from(v: f64) -> Self { ParamEnum::F64(v) }
}

impl<'a> From<Bodies<'a>> for ParamEnum<'a> {
    fn from(v: Bodies<'a>) -> Self { ParamEnum::Bodies(v) }
}

pub type Parameters<'a> = HashMap<&'a str, ParamEnum<'a>>;

pub trait ParamGetter<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

impl ParamGetter<i32> for ParamEnum<'_> {
    fn get(&self) -> &i32 {
        match self {
            Self::I32(v) => v,
            _ => panic!("Error")
        }
    }

    fn get_mut(&mut self) -> &mut i32 {
        match self {
            Self::I32(v) => v,
            _ => panic!("error")
        }
    }
}

impl ParamGetter<f64> for ParamEnum<'_> {
    fn get(&self) -> &f64 {
        match self {
            Self::F64(v) => v,
            _ => panic!("Error")
        }
    }

    fn get_mut(&mut self) -> &mut f64 {
        match self {
            Self::F64(v) => v,
            _ => panic!("error")
        }
    }
}

impl<'a> ParamGetter<Bodies<'a>> for ParamEnum<'a> {
    fn get(&self) -> &Bodies<'a> {
        match self {
            Self::Bodies(v) => v,
            _ => panic!("Error")
        }
    }

    fn get_mut(&mut self) -> &mut Bodies<'a> {
        match self {
            Self::Bodies(v) => v,
            _ => panic!("Error")
        }
    }
}
