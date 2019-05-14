pub fn run() {
    let nums1: Vec<i32> = [1, 3, 6, 9, 10].to_vec();
    let nums2: Vec<i32> = [10, 30, 60, 90, 100].to_vec();
    let total_len = nums1.len() + nums2.len();
    let (till, n) = [((total_len - 1) / 2, 2), (total_len / 2, 1)][total_len & 0x1];

    let mut iter = LambdaMerge::new(nums1, nums2);
    let sum = iter
        .enumerate()
        .skip(till)
        .map(|(_, x)| x)
        .take(n)
        .fold(0, |acc, x| acc + x);
    println!("{}", (sum as f64) / (n as f64));
}

struct LambdaMerge {
    iter1: std::vec::IntoIter<i32>,
    iter2: std::vec::IntoIter<i32>,
    num1: Option<i32>,
    num2: Option<i32>,
}

impl LambdaMerge {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> LambdaMerge {
        let mut iter = LambdaMerge {
            iter1: nums1.into_iter(),
            iter2: nums2.into_iter(),
            num1: None,
            num2: None,
        };
        iter.num1 = iter.iter1.next();
        iter.num2 = iter.iter2.next();
        iter
    }
}

impl Iterator for LambdaMerge {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let num = match (self.num1.take(), self.num2.take()) {
            (Some(num1), Some(num2)) if num1 <= num2 => {
                self.num1 = self.iter1.next();
                self.num2 = Some(num2);
                Some(num1)
            }
            (Some(num1), Some(num2)) => {
                self.num1 = Some(num1);
                self.num2 = self.iter2.next();
                Some(num2)
            }
            (Some(num1), None) => {
                self.num1 = self.iter1.next();
                Some(num1)
            }
            (None, Some(num2)) => {
                self.num2 = self.iter2.next();
                Some(num2)
            }
            (None, None) => None,
        };
        num
    }
}
