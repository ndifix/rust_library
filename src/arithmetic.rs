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

    fn add_positive_negative(&self, rhs: &Bigint) -> Bigint {
        if *self < -rhs.clone() {
            return -(-rhs.clone()).add_positive_negative(&-self.clone());
        }

        // すべての位について lhs >= rhs の状態に変形する無名関数
        let ready_to_sub = |lhs: &mut Bigint, rhs: &Bigint| {
            // lhs.number[i] >= rhs.number[i] の状態に変形する無名関数
            let ready_to_sub_impl = |lhs: &mut Bigint, rhs: &Bigint, i: usize| {
                if i >= rhs.number.len() {
                    return;
                }
                if lhs.number[i] >= rhs.number[i] {
                    return;
                }

                for j in i+1..rhs.number.len() {
                    if lhs.number[j] > rhs.number[j] {
                        for k in j..i+1 {
                            lhs.number[k] -= 1;
                            lhs.number[k-1] += Bigint::UPPER_BOUND;
                        }

                        return;
                    }
                }
            };

            for (i, _) in lhs.clone().number.iter().enumerate() {
                ready_to_sub_impl(lhs, rhs, i);
            }
        };

        let mut lhs = self.clone();
        ready_to_sub(&mut lhs, rhs);

        for (i, num) in lhs.number.iter_mut().enumerate() {
            if i >= rhs.number.len() {
                break;
            }

            *num -= rhs.number[i];
        }

        lhs.modify();
        lhs
    }
}

impl ops::Add<Bigint> for Bigint {
    type Output = Bigint;

    fn add(self, rhs: Bigint) -> Self::Output {
        match (&self.sign, &rhs.sign) {
            (Sign::Positive, Sign::Positive) => self.add_positive_positive(&rhs),
            (Sign::Positive, Sign::Negative) => self.add_positive_negative(&rhs),
            (Sign::Negative, Sign::Positive) => rhs.add_positive_negative(&self),
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

impl PartialOrd for Bigint {
    fn gt(&self, other: &Self) -> bool {
        if self.sign == Sign::Positive && other.sign == Sign::Negative {
            return true;
        };
        if self.sign == Sign::Negative && other.sign == Sign::Positive {
            return false;
        };
        if self.sign == Sign::Negative && other.sign == Sign::Negative {
            return (-other.clone()).gt(&-self.clone());
        };

        if self.number.len() != other.number.len() {
            return self.number.len() > other.number.len()
        };

        for (s, o) in self.number.iter().zip(other.number.iter()) {
            if s != o {
                return s > o;
            }
        };

        return false;
    }

    fn lt(&self, other: &Self) -> bool {
        other.ge(self)
    }

    fn ge(&self, other: &Self) -> bool {
        self.eq(other) || self.gt(other)
    }

    fn le(&self, other: &Self) -> bool {
        self.eq(other) || self.lt(other)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.gt(other) {
            Some(std::cmp::Ordering::Greater)
        } else if self.eq(other) {
            Some(std::cmp::Ordering::Equal)
        } else if self.lt(other) {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}
