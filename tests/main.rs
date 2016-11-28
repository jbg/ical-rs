
extern crate ical;
extern crate rustc_serialize;

#[cfg(feature = "rustc-serialize")]
mod test {


    use std::path::Path;
    use std::fs::File;
    use std::error::Error;
    use std::io::{BufReader, BufRead};


    use ical::parser::Parser;
    use rustc_serialize::json::ToJson;



    fn run_file(in_path_vcf: &Path, res_path_json: &Path) {
        let res_file = File::open(res_path_json).unwrap();
        let mut res_reader = BufReader::new(res_file).lines();

        let parser = Parser::from_path(in_path_vcf).unwrap();

        for elem in parser {
            let expected = match res_reader.next().unwrap() {
                Ok(val) => val,
                Err(err) => return assert!(false, err),
            };

            match elem {
                Ok(val) => {
                    let output = val.to_json();
                    assert_eq!(expected, format!("{}", output));
                }
                Err(err) => {
                    println!("tutu");
                    assert_eq!(expected, format!("{}", err.description()));
                }
            };


        }
    }

    #[test]
    fn invalid_path() {
        let path = Path::new("this is an invalid path");

        match Parser::from_path(path) {
            Ok(_) => assert!(false, "File should be invalid"),
            Err(err) => assert_eq!(err.description(), "entity not found"),
        }
    }

    #[test]
    fn vcard() {
        let in_path_vcf = Path::new("./tests/parser/vcard.vcf");
        let out_path_json = Path::new("./tests/parser/vcard.json");

        run_file(in_path_vcf, out_path_json);
    }


    #[test]
    fn vcard_error() {
        let in_path_vcf = Path::new("./tests/parser/vcard_error.vcf");
        let out_path_json = Path::new("./tests/parser/vcard_error.json");

        run_file(in_path_vcf, out_path_json);
    }
}