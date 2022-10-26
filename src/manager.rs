use std::fmt;
use std::fmt::Formatter;

use clap::Parser;
use rand::Rng;

use crate::error;
use crate::error::Error;

const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "0123456789";
const SPECIAL: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

#[derive(Parser)]
pub struct PasswordManager {
    #[arg(short, long)]
    upper_case: bool,

    #[arg(short, long)]
    lower_case: bool,

    #[arg(short, long)]
    digits: bool,

    #[arg(short, long)]
    special: bool,

    #[arg(long, default_value_t = 8)]
    min_len: u64,

    #[arg(long, default_value_t = 10)]
    max_len: u64,

    #[arg(short, long, default_value_t = 1)]
    count: u64,
}

impl PasswordManager {
    fn chars(&self) -> String {
        let mut result = String::new();

        if self.upper_case {
            result.push_str(UPPER_CASE)
        }

        if self.lower_case {
            result.push_str(LOWER_CASE)
        }

        if self.digits {
            result.push_str(DIGITS)
        }

        if self.special {
            result.push_str(SPECIAL)
        }

        result
    }

    fn len(&self) -> usize {
        let mut rng = rand::thread_rng();
        let password_len = rng.gen_range(self.min_len..self.max_len);

        password_len as usize
    }

    pub fn generate(&self) -> Result<PasswordResult, error::Error> {
        let mut rng = rand::thread_rng();
        let mut result = vec![];
        let chars = self.chars();

        if chars.is_empty() {
            return Err(Error::SelectCharsError)
        }

        for _ in 0..self.count {
            let mut password = String::new();

            for _ in 0..self.len() {
                let char_idx = rng.gen_range(0..chars.len());

                password.push(chars.as_bytes()[char_idx] as char);
            }

            result.push(password);
        }

        Ok(PasswordResult::new(result))
    }
}

#[derive(Debug)]
pub struct PasswordResult {
    pub passwords: Vec<String>,
}

impl PasswordResult {
    pub fn new(passwords: Vec<String>) -> Self {
        Self { passwords }
    }
}

impl fmt::Display for PasswordResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, password) in self.passwords.iter().enumerate() {
            write!(f, "{}) {}\r\n", idx + 1, password)?;
        }

        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars() {
        let mut pm = PasswordManager {
            upper_case: true,
            lower_case: true,
            digits: true,
            special: true,
            min_len: 10,
            max_len: 10,
            count: 1,
        };

        let chars = pm.chars();
        assert!(chars.contains(UPPER_CASE));
        assert!(chars.contains(LOWER_CASE));
        assert!(chars.contains(DIGITS));
        assert!(chars.contains(SPECIAL));

        pm.upper_case = false;
        let chars = pm.chars();

        assert!(!chars.contains(UPPER_CASE));
        assert!(chars.contains(LOWER_CASE));
        assert!(chars.contains(DIGITS));
        assert!(chars.contains(SPECIAL));

        pm.lower_case = false;
        let chars = pm.chars();

        assert!(!chars.contains(LOWER_CASE));
        assert!(!chars.contains(UPPER_CASE));
        assert!(chars.contains(DIGITS));
        assert!(chars.contains(SPECIAL));

        pm.digits = false;
        let chars = pm.chars();

        assert!(!chars.contains(DIGITS));
        assert!(!chars.contains(UPPER_CASE));
        assert!(!chars.contains(LOWER_CASE));
        assert!(chars.contains(SPECIAL));

        pm.special = false;
        let chars = pm.chars();

        assert!(!chars.contains(SPECIAL));
        assert!(!chars.contains(UPPER_CASE));
        assert!(!chars.contains(LOWER_CASE));
        assert!(!chars.contains(DIGITS));
    }

    #[test]
    fn test_len() {
        for i in 5..1000 {
            let max_len = i;
            let min_len = max_len - 5;
            let pm = PasswordManager {
                upper_case: false,
                lower_case: false,
                digits: false,
                special: false,
                min_len,
                max_len,
                count: 10,
            };
            let password_len = pm.len();

            assert!(min_len <= password_len as u64);
            assert!(max_len >= password_len as u64);
        }
    }

    #[test]
    fn test_generate() -> Result<(), error::Error> {
        let pm = PasswordManager {
            upper_case: true,
            lower_case: true,
            digits: true,
            special: true,
            min_len: 10,
            max_len: 15,
            count: 10,
        };

        for _ in 0..1000 {
            let result = pm.generate()?;

            assert_eq!(result.passwords.len(), 10);

            for password in result.passwords {
                assert!(password.len() >= 10);
                assert!(password.len() <= 15);
            }
        }

        Ok(())
    }

    #[test]
    fn test_generate_error() {
        let pm = PasswordManager {
            upper_case: false,
            lower_case: false,
            digits: false,
            special: false,
            min_len: 10,
            max_len: 15,
            count: 10,
        };

        assert_eq!(pm.generate().unwrap_err(), error::Error::SelectCharsError);
    }
}
