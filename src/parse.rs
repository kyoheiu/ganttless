use super::ganttless::MyError;
use chrono::NaiveDate;

pub fn input_to_i(input: String) -> Result<(i64, i64), MyError> {
    let split: Vec<&str> = input.split(':').collect();
    let mut begin = split[0].trim().parse::<i64>()?;
    let mut end = split[1].trim().parse::<i64>()?;
    if end < begin {
        std::mem::swap(&mut begin, &mut end);
    }
    Ok((begin, end))
}

pub fn input_to_date(input: String) -> Result<(NaiveDate, NaiveDate), MyError> {
    let split: Vec<&str> = input.split(':').collect();
    let mut begin = split[0].trim().parse::<NaiveDate>()?;
    let mut end = split[1].trim().parse::<NaiveDate>()?;
    if end < begin {
        std::mem::swap(&mut begin, &mut end);
    }
    Ok((begin, end))
}

pub fn input_to_tuple(input: String) -> Result<(String, String), MyError> {
    let v: Vec<&str> = input.split('=').collect();
    if v.len() != 2 {
        Err(MyError::InputError)
    } else {
        Ok((v[0].to_string(), v[1].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_to_i_test() -> Result<(), MyError> {
        let input = "-2 -> -10".to_string();
        assert_eq!((-10, -2), input_to_i(input)?);
        let input_reverse = "-10 ->-2".to_string();
        assert_eq!((-10, -2), input_to_i(input_reverse)?);
        let input_reverse = "100->120".to_string();
        assert_eq!((100, 120), input_to_i(input_reverse)?);
        Ok(())
    }
}
