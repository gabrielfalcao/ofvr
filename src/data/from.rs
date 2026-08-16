use std::iter::IntoIterator;

use super::core::Data;

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Data {
        Data::new(data)
    }
}
impl From<&Vec<u8>> for Data {
    fn from(data: &Vec<u8>) -> Data {
        let data = data.clone();
        Data::new(data)
    }
}
impl From<&[u8]> for Data {
    fn from(data: &[u8]) -> Data {
        let data = data.to_vec();
        Data::new(data)
    }
}

impl FromIterator<u8> for Data {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut c = Data::new(Vec::new());

        for i in iter {
            c.push(i);
        }

        c
    }
}
impl <'a>FromIterator<&'a u8> for Data {
    fn from_iter<I: IntoIterator<Item = &'a u8>>(iter: I) -> Self {
        let mut c = Data::new(Vec::new());

        for i in iter {
            c.push(*i);
        }

        c
    }
}
