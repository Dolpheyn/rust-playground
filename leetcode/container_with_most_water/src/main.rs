use std::cmp::max;
use std::cmp::min;
fn main() {
    //let max = max_area(vec![1,8,6,2,5,4,8,3,7]);
    let max = max_area(vec![2,3,4,5,18,17,6]);
    //println!("{}", max);
}
fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let right = height.len();
    let mut max_area = 0;
    
    loop {
        let width = (right - left - 1) as i32;
        let h = min(height[left], height[right - 1]);
        println!("{}", h * width);
        max_area = max(max_area, h * width);
        left += 1;
        if left >= right {
            break;
        }
    }
    max_area
}
