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
    const DIGIT: usize = 9;
    const UPPER_BOUND: u64 = 10_u64.pow(Bigint::DIGIT as u32);

    #[allow(dead_code)]
    fn from_int(n: i32) -> Bigint {
        Bigint {
            sign: n >= 0,
            number: vec![n.abs() as u64, 0],
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut str = match self.sign {
            true => String::new(),
            false => String::from("-"),
        };

        for (i, num) in self.number.clone().iter().rev().enumerate() {
            if i == 0 {
                continue;
            }
            if i == 1 {
                str += num.to_string().as_str();
            } else {
                str += format!("{:0>1$}", num, Bigint::DIGIT).as_str();
            }
        }
        str
    }

    fn modify(&mut self) {
        let mut i: usize = 0;
        while i < self.number.len() {
            if self.number[i] >= Bigint::UPPER_BOUND {
                let div = self.number[i] / Bigint::UPPER_BOUND;
                self.number[i] %= Bigint::UPPER_BOUND;
                self.number[i+1] += div;

                if let Some(num) = self.number.last() {
                    if *num != 0 {
                        self.number.push(0);
                        println!("pushed");
                    }
                }
            }

            i += 1;
        }
    }

    fn add_positive_positive(&self, rhs: &Bigint) -> Bigint {
        if self.number.len() < rhs.number.len() {
            return rhs.add_positive_positive(self);
        };

        let mut ret: Bigint = self.clone();
        for (i, num) in rhs.number.iter().enumerate() {
            ret.number[i] += num;
        }

        ret.modify();
        ret
    }

    fn add_negative_negative(&self, rhs: &Bigint) -> Bigint {
        self.add_positive_positive(rhs)
    }
}

impl ops::Add<Bigint> for Bigint {
    type Output = Bigint;

    fn add(self, rhs: Bigint) -> Self::Output {
        match (self.sign, rhs.sign) {
            (true, true)   => self.add_positive_positive(&rhs),
            (true, false)  => panic!("no implmentation"),
            (false, true)  => panic!("no implmentation"),
            (false, false) => self.add_negative_negative(&rhs),
        }
    }
}
