#[cfg(test)]
mod test;

struct Bigint {
    #[allow(dead_code)]
    sign: bool,
    #[allow(dead_code)]
    number: Vec<u64>,
}

impl Bigint {
    #[allow(dead_code)]
    fn from_int(n: i32) -> Bigint {
        Bigint {
            sign: n >= 0,
            number: vec![n.abs() as u64],
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut str = match self.sign {
            true => String::new(),
            false => String::from("-"),
        };

        str += self.number[0].to_string().as_str();

        str
    }
}
