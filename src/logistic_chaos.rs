pub struct LogisticChaosIter {
    x: f64,
    u: f64,
}

impl LogisticChaosIter {
    pub fn new(x: f64, u: f64) -> Self {
        if (u < 0. || u > 4.) && (x < 0. || x > 1.) {
            panic!("Invalid u or x");
        }
        Self { x, u }
    }
}

impl Iterator for LogisticChaosIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.x = self.u * self.x * (1. - self.x);
        Some(self.x)
    }
}
