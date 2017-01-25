
mod values;

use std::io::BufRead;
use std::cell::RefCell;

use line::parser::LineParser;
use line::reader::LineReader;
use ParseError;
use self::values::*;

#[derive(Debug, Clone)]
pub struct Contact {
    emails: Vec<Email>,
}

impl Contact {
    pub fn new() -> Contact {
        Contact{
            emails: vec!{}
        }
    }

    pub fn add_email(&mut self, email: Email) {
        self.emails.push(email);
    }
}

pub struct VcardParser<B> {
    line_parser: RefCell<LineParser<B>>,
}

impl<B: BufRead> VcardParser<B> {
    pub fn new(reader: B) -> VcardParser<B> {
        let line_reader = LineReader::new(reader);
        let line_parser = LineParser::new(line_reader);

        VcardParser { line_parser: RefCell::new(line_parser) }
    }


    /// Read the next line and check if it's a valid VCARD start.
    fn check_header(&mut self) -> Result<Option<()>, ParseError> {
        let mut line_parser = self.line_parser.borrow_mut();

        let mut line;

        line = match line_parser.next() {
            Some(val) => val?,
            None => return Ok(None),
        };

        if line.name != "BEGIN" || line.value != "VCARD" || line.params != None {
            return Err(ParseError::MissingHeader);
        }

        line = match line_parser.next() {
            Some(val) => val?,
            None => return Ok(None),
        };

        if line.name != "VERSION" || line.value != "4.0" || line.params != None {
            return Err(ParseError::MissingHeader);
        }

        Ok(Some(()))
    }


    /// Parse a full contact.
    fn parse(&mut self) -> Result<Contact, ParseError> {
        let mut line_parser = self.line_parser.borrow_mut();

        let mut contact = Contact::new();

        while let Some(res) = line_parser.next() {
            let line = match res {
                Ok(line) => line,
                Err(err) => return Err(err)
            };

            println!("{:?}", line);

            match line.name.as_str() {
                "EMAIL" => contact.add_email(values::Email::from_line_parsed(&line)?),
                "ANNIVERSARY" => contact.set_anniversary(values::Anniversary::from_line_parsed(&line)?),
                // _ => return Err(ParseError::NotImplemented)
                _ => println!("Not implemented: {}", line.name)
            };

            println!("{:?}", contact);

            if line.name == "END" {
                break;
            }
        }

        Ok(contact)
    }
}


impl<B: BufRead> Iterator for VcardParser<B> {
    type Item = Result<Contact, ParseError>;

    fn next(&mut self) -> Option<Result<Contact, ParseError>> {
        match self.check_header() {
            Ok(res) => {
                if res == None {
                    return None;
                }
            }
            Err(err) => return Some(Err(err)),
        };

        let result = match self.parse() {
            Ok(contact) => Ok(contact),
            Err(err) => Err(err),
        };

        Some(result)
    }
}
