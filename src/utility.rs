use std::collections::VecDeque;
use std::str::FromStr;

pub enum UtilityError {
    ConversionError
}

pub fn convert_vec<DataT: FromStr>(buffer: &VecDeque<String>) -> Result<Vec<DataT>, UtilityError> {
    let mut converted_parameters = Vec::<DataT>::new();
    converted_parameters.reserve(buffer.len());

    for i in 0..buffer.len() {
        if let Ok(new_parameter) = buffer[i].parse::<DataT>() {
            converted_parameters.push(new_parameter);
        } else {
            return Err(UtilityError::ConversionError);
        }
    }

    Ok(converted_parameters)
}