extern crate base64;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate serde_json;

pub mod selenium;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
