
use chrono::*;

use line::parser::LineParsed;
use ParseError;


#[derive(Debug, Clone)]
pub struct Email {
    value: String,
    tag: Option<String>,
    pref: Option<i32>,
}

impl Email {
    // Create an email from a `LineParsed`
    //
    // It not use all the parameters, only the "TYPE" and "PREF".
    pub fn from_line_parsed(line_parsed: &LineParsed) -> Result<Email, ParseError> {
        let mut tag = None;
        let mut pref = None;

        if let Some(ref params) = line_parsed.params {
            for param in params {
                match param.0.as_str() {
                    "TYPE" => tag = Some(param.1[0].to_lowercase()),
                    "PREF" => {
                        let res = param.1[0].parse::<i32>();

                        if res.is_err() {
                            return Err(ParseError::InvalidParamFormat)
                        } else {
                            pref = Some(res.unwrap());
                        }
                    }
                    _ => (),
                };
            }
        }


        Ok(Email {
            value: line_parsed.value.clone(),
            tag: tag,
            pref: pref,
        })
    }
}

pub struct Anniversary(chrono::DateTime<UTC>);
