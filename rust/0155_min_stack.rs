struct MinStack(Vec<(i32, i32)>); // using tuple struct

impl MinStack {
    fn new() -> Self {
        MinStack(vec![])
    }

    fn push(&mut self, val: i32) {
        let min = self.0.last().map(|(_, m)| *m).unwrap_or(val); // get min from last tuple if not empty. val otherwise
        self.0.push((val, val.min(min))); // push tuple of (val, min)
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn top(&self) -> i32 {
        self.0.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.0.last().unwrap().1
    }
}