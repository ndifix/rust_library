use std::ops;

#[cfg(test)]
mod test;

#[derive(Clone)]
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

impl ops::Add<Bigint> for Bigint {
    type Output = Bigint;

    fn add(self, rhs: Bigint) -> Self::Output {
        if self.sign != rhs.sign {
            panic!("no implmentation");
        };

        if self.number.len() < rhs.number.len() {
            return rhs.add(self);
        };

        let mut ret: Bigint = self.clone();
        for (i, num) in rhs.number.iter().enumerate() {
            ret.number[i] += num;
        }

        ret
    }
}
