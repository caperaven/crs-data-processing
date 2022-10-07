use crate::traits::AggregateNumber;

pub struct Max {
    pub value: f64
}

impl Max {
    pub fn new() -> Max {
        Max {
            value: f64::MIN
        }
    }
}

impl AggregateNumber for Max {
    fn add(&mut self, value: f64) {
        if value > self.value {
            self.value = value;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::aggregate::numbers::max::Max;
    use crate::traits::AggregateNumber;

    #[test]
    fn min() {
        let mut max = Max::new();
        max.add(10.);
        max.add(100.);
        assert_eq!(max.value, 100.);
    }
}