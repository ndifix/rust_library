use std::ops;

#[cfg(test)]
mod test;

#[derive(Clone)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Clone)]
struct Bigint {
    #[allow(dead_code)]
    sign: Sign,
    #[allow(dead_code)]
    number: Vec<u64>,
}

impl Bigint {
    const DIGIT: usize = 9;
    const UPPER_BOUND: u64 = 10_u64.pow(Bigint::DIGIT as u32);

    #[allow(dead_code)]
    fn from_int(n: i32) -> Bigint {
        let sign = if n >= 0 {
            Sign::Positive
        } else {
            Sign::Negative
        };

        Bigint {
            sign,
            number: vec![n.abs() as u64, 0],
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut str = match self.sign {
            Sign::Positive => String::new(),
            Sign::Negative => String::from("-"),
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
        match (&self.sign, &rhs.sign) {
            (Sign::Positive, Sign::Positive) => self.add_positive_positive(&rhs),
            (Sign::Positive, Sign::Negative) => panic!("no implmentation"),
            (Sign::Negative, Sign::Positive) => panic!("no implmentation"),
            (Sign::Negative, Sign::Negative) => self.add_negative_negative(&rhs),
        }
    }
}

impl ops::Neg for Bigint {
    type Output = Bigint;

    fn neg(self) -> Self::Output {
        let mut bg = self.clone();
        bg.sign = match &bg.sign {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        };

        bg
    }
}

impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Sign::Positive, Sign::Positive) => true,
            (Sign::Positive, Sign::Negative) => false,
            (Sign::Negative, Sign::Positive) => false,
            (Sign::Negative, Sign::Negative) => true,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq for Bigint {
    fn eq(&self, other: &Self) -> bool {
        !self.ne(other)
    }

    fn ne(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return true;
        };

        if self.number.len() != other.number.len() {
            return  true;
        };

        for i in 0..self.number.len() {
            if self.number[i] != other.number[i] {
                return true;
            }
        };

        return false;
    }
}
