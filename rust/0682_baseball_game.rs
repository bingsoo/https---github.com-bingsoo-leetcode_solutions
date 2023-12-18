impl Solution {
    pub fn cal_points(o: Vec<String>) -> i32 {
        let mut r = Vec::new();
        for s in o.iter() {
            match s.as_str() {
                "C" => {r.pop();},
                "D" => {r.push(r[r.len()-1]*2);},
                "+" => {r.push(r[r.len()-1]+r[r.len()-2]);},
                d => r.push(d.parse::<i32>().unwrap()),
            }
        }
        r.iter().sum()
    }
}