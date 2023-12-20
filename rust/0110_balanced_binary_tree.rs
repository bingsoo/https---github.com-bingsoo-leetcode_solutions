use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(n: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            n.map(|n| {
                let n = n.borrow();
                let l = dfs(n.left.clone())?; // early return if None
                let r = dfs(n.right.clone())?;
                let h = l.max(r) + 1;
                ((l - r).abs() <= 1).then(|| h) // return h if balanced else None
            }).unwrap_or(Some(-1))
        }
        dfs(r).is_some()
    }
}

// more straightforward solution
// impl Solution {
//     pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
//             match root {
//                 Some(root) => {
//                     let (left, right) = (
//                         dfs(root.borrow().left.clone()),
//                         dfs(root.borrow().right.clone()),
//                     );
//                     let is_balanced = left.0 && right.0 && (left.1 - right.1).abs() <= 1;
//                     (is_balanced, cmp::max(left.1, right.1) + 1)
//                 }
//                 None => (true, 0),
//             }
//         }
//         dfs(root).0
//     }
// }