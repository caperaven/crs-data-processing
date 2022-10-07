use crate::traits::AggregateNumber;

pub struct Min {
    pub value: f64
}

impl Min {
    pub fn new() -> Min {
        Min {
            value: f64::MAX
        }
    }
}

impl AggregateNumber for Min {
    fn add(&mut self, value: f64) {
        if value < self.value {
            self.value = value;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::numbers::min::Min;
    use crate::traits::AggregateNumber;

    #[test]
    fn min() {
        let mut min = Min::new();
        min.add(100.);
        min.add(20.);
        assert_eq!(min.value, 20.);
    }
}