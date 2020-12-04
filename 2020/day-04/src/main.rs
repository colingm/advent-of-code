use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref PASSPORT_FIELD_RE: Regex = Regex::new(r"(\w{3}):([#\w\d]+)").unwrap();
    static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[derive(Default)]
struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>
}

impl Passport {
    pub fn new(line: &str) -> Self {
        let mut passport = Self {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None
        };

        line.split_whitespace()
            .for_each(|field| {
                let results = PASSPORT_FIELD_RE.captures(field).unwrap();
                let key = results.get(1).unwrap().as_str();
                let value = results.get(2).unwrap().as_str().to_string();

                match key {
                    "byr" => passport.birth_year = Some(value.parse().unwrap()),
                    "iyr" => passport.issue_year = Some(value.parse().unwrap()),
                    "eyr" => passport.expiration_year = Some(value.parse().unwrap()),
                    "hgt" => passport.height = Some(value),
                    "hcl" => passport.hair_color = Some(value),
                    "ecl" => passport.eye_color = Some(value),
                    "pid" => passport.passport_id = Some(value),
                    "cid" => passport.country_id = Some(value),
                    _ => (),
                };
            });

        passport
    }

    fn is_valid_number(&self, value: usize, min: usize, max: usize) -> bool {
        min <= value && value <= max
    }

    pub fn is_valid_birth_year(&self) -> bool {
        self.birth_year.is_some() && self.is_valid_number(self.birth_year.unwrap(), 1920, 2002)
    }

    pub fn is_valid_issue_year(&self) -> bool {
        self.issue_year.is_some() && self.is_valid_number(self.issue_year.unwrap(), 2010, 2020)
    }

    pub fn is_valid_expiration_year(&self) -> bool {
        self.expiration_year.is_some() && self.is_valid_number(self.expiration_year.unwrap(), 2020, 2030)
    }

    pub fn is_valid_height(&self) -> bool {
        if self.height.as_ref().is_none() {
            return false;
        }

        match HEIGHT_RE.captures(self.height.as_ref().unwrap().as_str()) {
            Some(results) => {
                match results.get(2).unwrap().as_str() {
                    "in" => self.is_valid_number(results.get(1).unwrap().as_str().parse().unwrap(), 59, 76),
                    "cm" => self.is_valid_number(results.get(1).unwrap().as_str().parse().unwrap(), 150, 193),
                    _ => false,
                }
            },
            None => false
        }
    }

    pub fn is_valid_hair_color(&self) -> bool {
        if self.hair_color.as_ref().is_none() {
            return false;
        }

        HAIR_COLOR_RE.is_match(self.hair_color.as_ref().unwrap().as_str())
    }

    pub fn is_valid_eye_color(&self) -> bool {
        if self.eye_color.as_ref().is_none() {
            return false;
        }

        EYE_COLOR_RE.is_match(self.eye_color.as_ref().unwrap().as_str())
    }

    pub fn is_valid_passport_id(&self) -> bool {
        if self.passport_id.as_ref().is_none() {
            return false;
        }

        PASSPORT_ID_RE.is_match(self.passport_id.as_ref().unwrap().as_str())
    }

    pub fn is_complete(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_birth_year()
            && self.is_valid_issue_year()
            && self.is_valid_expiration_year()
            && self.is_valid_height()
            && self.is_valid_hair_color()
            && self.is_valid_eye_color()
            && self.is_valid_passport_id()
    }
}

fn get_passports(filename: &str) -> Vec<Passport> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(Passport::new)
        .collect()
}

fn count_complete_passports(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|passport| passport.is_complete())
        .count()
}

fn count_valid_passports(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|passport| passport.is_valid())
        .count()
}

fn main() {
    let passports = get_passports("input.txt");

    println!("Part 1 Result: {}", count_complete_passports(&passports));
    println!("Part 2 Result: {}", count_valid_passports(&passports));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_complete() {
        let passports = get_passports("test.txt");

        assert_eq!(count_complete_passports(&passports), 2);
    }

    #[test]
    fn test_invalid_passports() {
        let passports = get_passports("test_invalid.txt");

        assert_eq!(count_valid_passports(&passports), 0);
    }

    #[test]
    fn test_valid_passports() {
        let passports = get_passports("test_valid.txt");

        assert_eq!(count_valid_passports(&passports), 4);
    }
}
