use crate::{comp, err, libs::complex::Comp};

pub fn lg(a: Option<&str>) -> Result<Comp, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<f64>() {
            return Ok(comp!(f64::log2(a)));
        }
        return err!("error.invalid_argument");
    }
    err!("error.need_more_arguments")
}

pub fn ln(a: Option<&str>) -> Result<Comp, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<f64>() {
            return Ok(comp!(f64::ln(a)));
        }
        return err!("error.invalid_argument");
    }
    err!("error.need_more_arguments")
}

pub fn sqrt(a: Option<&str>) -> Result<Comp, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<Comp>() {
            return Ok(Comp::sqrt(a));
        }
        return err!("error.invalid_argument");
    }
    err!("error.need_more_arguments")
}

pub fn cbrt(a: Option<&str>) -> Result<Comp, String> {
    if let Some(a) = a {
        if let Ok(a) = a.parse::<f64>() {
            return Ok(comp!(f64::cbrt(a)));
        }
        return err!("error.invalid_argument");
    }
    err!("error.need_more_arguments")
}
