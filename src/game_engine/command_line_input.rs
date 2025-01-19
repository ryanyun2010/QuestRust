use dialog::DialogBox;

use crate::{error::PError, perror};

pub fn prompt_float(prompt: &str) -> Result<f32, PError> {
    let prompt_res = dialog::Input::new(prompt).show();
    if !prompt_res.is_ok() {
        return Err(perror!(InputFailed, "prompt: {:?} failed to get input?", prompt));
    }
    let new_x_potentially = prompt_res.unwrap();
    if new_x_potentially.is_none() {
        return Err(perror!(InputFailed, "prompt: {:?} wasn't valid or was blank?", prompt));
    }
    let new_x_res = new_x_potentially.unwrap().parse::<f32>();
    if !new_x_res.is_ok() {
        return Err(perror!(InputFailed, "prompt: {:?} input wasn't a float?", prompt));
    }
    Ok(new_x_res.unwrap())
}

pub fn prompt_string(prompt: &str) -> Result<String, PError> {
    let prompt_res = dialog::Input::new(prompt).show();
    if !prompt_res.is_ok() {
        return Err(perror!(InputFailed, "prompt: {:?} failed to get input?", prompt));
    }
    let new_x_potentially = prompt_res.unwrap();
    if new_x_potentially.is_none() {
        return Err(perror!(InputFailed, "prompt: {:?} wasn't valid or was blank?", prompt));
    }
    Ok(new_x_potentially.unwrap())
}