use crate::traits::AggregateNumber;

pub struct Sum {
    pub total: f64
}

impl Sum {
    pub fn new() -> Sum {
        Sum {
            total: 0.
        }
    }
}

impl AggregateNumber for Sum {
    fn add(&mut self, value: f64) {
        self.total += value;
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::numbers::sum::Sum;
    use crate::traits::AggregateNumber;

    #[test]
    fn sum() {
        let mut sum = Sum::new();
        sum.add(1.);
        sum.add(2.);
        assert_eq!(sum.total, 3.);
    }
}
