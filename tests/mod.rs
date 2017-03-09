
use std::io::Write;
use std::io::Result;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
struct StubFile {
    pub line: Rc<RefCell<String>>
}

impl Write for StubFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut content = buf.clone();

        content.read_to_string(&mut self.line.borrow_mut())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl StubFile {
    pub fn new(line: Rc<RefCell<String>>) -> Self {
        StubFile {
            line: line
        }
    }
}


#[cfg(feature = "property")]
pub mod property {
    extern crate ical;

    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    #[test]
    fn ical() {
        let input = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/ical_property.res").unwrap()).lines();


        let reader = ical::PropertyParser::from_reader(input);

        for res in reader {
            let calendar = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", calendar);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }


    #[test]
    fn vcard() {
        let input = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/vcard_property.res").unwrap()).lines();


        let reader = ical::PropertyParser::from_reader(input);

        for res in reader {
            let contact = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", contact);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn errors() {
        let input = BufReader::new(File::open("./tests/ressources/property_error.vcf").unwrap());

        let mut valids =
            BufReader::new(File::open("./tests/ressources/property_error.res").unwrap()).lines();


        let reader = ical::PropertyParser::from_reader(input);

        for res in reader {
            let error = match res {
                Ok(res) => panic!("Should return an error: {:?}", res),
                Err(err) => err,
            };

            let output = format!("{}", error);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }
}


#[cfg(feature = "line")]
pub mod line {
    extern crate ical;

    use std::io::{BufReader, BufRead, Read};
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::fs::File;

    use super::StubFile;

    #[test]
    fn ical() {
        let input = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

        let mut valids = BufReader::new(File::open("./tests/ressources/ical_line.res").unwrap())
            .lines();


        let reader = ical::LineReader::new(input);

        for line in reader {
            let output = format!("{:?}", line);
            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn vcard() {
        // File Input.
        let input = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

        // Read.
        let mut res_read = BufReader::new(File::open("./tests/ressources/vcard_line_read.res").unwrap())
            .lines();
        let reader = ical::LineReader::new(input);

        // Write
        let buf = Rc::new(RefCell::new(String::new()));
        let stub_file = StubFile::new(buf);
        let mut writer = ical::LineWriter::new(stub_file.clone());
        let mut res_write = BufReader::new(File::open("./tests/ressources/vcard_line_write.res").unwrap());

        for line in reader {
            // Check Reader.
            let output = format!("{:?}", line);
            assert_eq!(output, res_read.next().unwrap().unwrap());

            // Run Writer.
            writer.write(&line).unwrap();
        }

        let mut buf_res = String::new();
        res_write.read_to_string(&mut buf_res).unwrap();

        let content_ref = stub_file.line.borrow();
        let mut buf_out = content_ref.as_str().lines();

        for line in buf_res.lines() {
            assert_eq!(line, buf_out.next().unwrap());
        }
    }
}

#[cfg(any(feature = "ical", feature = "vcard"))]
pub mod parser {
    extern crate ical;

    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    #[test]
    fn ical() {
        let input = BufReader::new(File::open("./tests/ressources/ical_input.ics").unwrap());

        let mut valids = BufReader::new(File::open("./tests/ressources/ical_parser.res").unwrap())
            .lines();


        let reader = ical::IcalParser::new(input);

        for res in reader {
            let calendar = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };

            let output = format!("{:?}", calendar);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }

    #[test]
    fn vcard() {
        let input = BufReader::new(File::open("./tests/ressources/vcard_input.vcf").unwrap());

        let mut valids = BufReader::new(File::open("./tests/ressources/vcard_parser.res").unwrap())
            .lines();


        let reader = ical::VcardParser::new(input);

        for res in reader {
            let contact = match res {
                Ok(res) => res,
                Err(err) => panic!("Throw errror: {}", err),
            };


            let output = format!("{:?}", contact);

            assert_eq!(output, valids.next().unwrap().unwrap());
        }
    }
}
