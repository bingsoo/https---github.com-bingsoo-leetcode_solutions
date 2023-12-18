// manually binary search
//use std::cmp::Ordering;
// impl Solution {
//     pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//         let mut min: i32 = 0;
//         let mut max: i32 = (nums.len()-1) as i32;
//         while min <= max {
//             let mid = (min+max)/2;
//             match nums[mid as usize].cmp(&target) {
//                 Ordering::Equal => return mid,
//                 Ordering::Less => min=mid+1,
//                 Ordering::Greater => max = mid-1,
//             }
//         }
//         -1
//     }
// }

// one line solution
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).map(|x| x as i32).unwrap_or(-1)
    }
}