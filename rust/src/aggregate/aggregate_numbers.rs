pub struct NumberAggregator {
    name: String,
    total: f64,
    min: f64,
    max: f64,
    count: f64
}

impl NumberAggregator {
    pub fn new(name: String) -> NumberAggregator {
        NumberAggregator {
            name,
            total: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0.0
        }
    }
    
    pub fn add(&mut self, value: f64) {
        self.total += value;

        if self.min > value {
            self.min = value;
        }

        if self.max < value {
            self.max = value;
        }

        self.count += 1.;
    }

    pub fn value(&self) {
        let min = self.min;
        let max = self.max;
        let sum = self.total;
        let ave = self.total / self.count;
    }
}