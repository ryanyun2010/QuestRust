use dialog::DialogBox;

pub fn prompt_float(prompt: &str) -> Option<f32> {
    let prompt_res = dialog::Input::new(prompt).show();
    if !prompt_res.is_ok() {
        return None;
    }
    let new_x_potentially = prompt_res.unwrap();
    if new_x_potentially.is_none() {
        return None;
    }
    let new_x_res = new_x_potentially.unwrap().parse::<f32>();
    if !new_x_res.is_ok() {
        return None;
    }
    Some(new_x_res.unwrap())
}

pub fn prompt_string(prompt: &str) -> Option<String> {
    let prompt_res = dialog::Input::new(prompt).show();
    if !prompt_res.is_ok() {
        return None;
    }
    let new_x_potentially = prompt_res.unwrap();
    if new_x_potentially.is_none() {
        return None;
    }
    Some(new_x_potentially.unwrap())
}