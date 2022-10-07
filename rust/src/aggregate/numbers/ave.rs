use crate::traits::AggregateNumber;

pub struct Ave {
    count: f64,
    total: f64,
    pub value: f64
}

impl Ave {
    pub fn new() -> Ave {
        Ave {
            count: 0.0,
            total: 0.0,
            value: f64::MAX
        }
    }
}

impl AggregateNumber for Ave {
    fn add(&mut self, value: f64) {
        self.count += 1.;
        self.total += value;
        self.value = self.total / self.count;
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::numbers::ave::Ave;
    use crate::traits::AggregateNumber;

    #[test]
    fn min() {
        let mut ave = Ave::new();
        ave.add(100.);
        ave.add(20.);
        assert_eq!(ave.value, 60.);
    }
}