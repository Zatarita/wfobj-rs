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

pub fn convert_to_usize(buffer: &VecDeque<String>) -> Result<Vec<usize>, UtilityError> {
    let mut converted_parameters = Vec::<usize>::new();
    converted_parameters.reserve(buffer.len());

    for i in 0..buffer.len() {
        if let Ok(new_parameter) = buffer[i].parse::<usize>() {
            converted_parameters.push(new_parameter);
        } else {
            return Err(UtilityError::ConversionError);
        }
    }

    Ok(converted_parameters)
}