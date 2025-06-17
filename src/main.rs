use std::{env, str::FromStr};

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum Unit {
    Len,
    Mass,
    Temp,
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "len" => Ok(Unit::Len),
            "mass" => Ok(Unit::Mass),
            "Temp" => Ok(Unit::Temp),
            _ => Err("Unknown unit".to_owned()),
        }
    }
}

fn calculate(unit: Unit, value: f64, from: &String, to: &String) -> Result<f64, String> {
    match unit {
        Unit::Len => calculate_linear(value, from, to, get_len_factor),
        Unit::Mass => todo!(),
        Unit::Temp => todo!(),
    }
}

fn get_len_factor(value: &String) -> Result<f64, String> {
    match value.to_lowercase().as_str() {
        "m" | "meter" | "meters" => Ok(1.0),
        "km" => Ok(1000.0),
        "cm" => Ok(0.01),
        "mm" => Ok(0.001),
        "mile" => Ok(1609.34),
        "ft" => Ok(0.3048),
        "in" => Ok(0.0254),
        _ => Err(format!("Unknown unit: {}", value)),
    }
}

fn calculate_linear<F>(value: f64, from: &String, to: &String, get_factor: F) -> Result<f64, String>
where
    F: Fn(&String) -> Result<f64, String>,
{
    let from_factor = get_factor(from)?;
    let to_factor = get_factor(to)?;
    let result = value * from_factor / to_factor;
    Ok(result)
}

fn parse_unit(unit: &String) -> Result<Unit, String> {
    let result = unit.trim().parse()?;
    Ok(result)
}

fn parse_value(value: &String) -> Result<f64, String> {
    let result = value.trim().parse::<f64>().map_err(|e| e.to_string())?;
    Ok(result)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let unit = &args[1];
    let value = &args[2];
    let from = &args[3];
    let to = &args[4];

    let unit = parse_unit(unit)?;
    let value = parse_value(value)?;

    let result = calculate(unit, value, from, to)?;

    println!("result {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_len() {
        let result = calculate(Unit::Len, 1.0, &String::from("m"), &String::from("cm"));
        assert_eq!(Ok(100.0), result);
    }

    #[test]
    fn test_conversion_with_unknown_unit_returns_err() {
        let value = 10.0;
        let from = "m".to_string();
        let to = "furlong".to_string();

        let result = calculate(Unit::Len, value, &from, &to);

        assert!(
            result.is_err(),
            "The function should have returned an error, but it returned Ok."
        );

        if let Err(e) = result {
            assert_eq!(e, "Unknown unit: furlong");
        }
    }

    #[test]
    fn test_parsing_string_to_f64() {
        let res = parse_value(&"20.0".to_owned());

        assert!(
            res.is_ok(),
            "Function should have returned Ok but returned Err"
        );

        if let Ok(res) = res {
            assert_eq!(20.0, res);
        }
    }

    #[test]
    fn test_parsing_unit() {
        let res = parse_unit(&String::from("len"));

        if let Ok(res) = res {
            assert_eq!(res, Unit::Len);
        }
    }
}
