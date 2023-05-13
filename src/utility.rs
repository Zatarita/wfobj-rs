use std::collections::VecDeque;

pub enum UtilityError {
    ConversionError
}

pub fn convert_to_f32(buffer: &VecDeque<String>) -> Result<Vec<f32>, UtilityError> {
    let mut converted_parameters = Vec::<f32>::new();
    converted_parameters.reserve(buffer.len());

    for i in 0..buffer.len() {
        if let Ok(new_parameter) = buffer[i].parse::<f32>() {
            converted_parameters.push(new_parameter);
        } else {
            return Err(UtilityError::ConversionError);
        }
    }

    Ok(converted_parameters)
}