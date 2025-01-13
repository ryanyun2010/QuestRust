use dialog::DialogBox;
use anyhow::{anyhow, Error};

pub fn prompt_float(prompt: &str) -> Result<f32, anyhow::Error> {
    let prompt_res = dialog::Input::new(prompt).show();
    if !prompt_res.is_ok() {
        return Err(anyhow!("prompt: {:?} failed to get input?", prompt));
    }
    let new_x_potentially = prompt_res.unwrap();
    if new_x_potentially.is_none() {
        return Err(anyhow!("prompt: {:?} input wasn't valid or was blank?", prompt));
    }
    let new_x_res = new_x_potentially.unwrap().parse::<f32>();
    if !new_x_res.is_ok() {
        return Err(anyhow!("prompt: {:?} input wasn't a float?", prompt));
    }
    Ok(new_x_res.unwrap())
}

pub fn prompt_string(prompt: &str) -> Result<String, anyhow::Error> {
    let prompt_res = dialog::Input::new(prompt).show();
    if !prompt_res.is_ok() {
        return Err(anyhow!("prompt: {:?} failed to get input?", prompt));
    }
    let new_x_potentially = prompt_res.unwrap();
    if new_x_potentially.is_none() {
        return Err(anyhow!("prompt: {:?} input wasn't valid or was blank?", prompt));
    }
    Ok(new_x_potentially.unwrap())
}