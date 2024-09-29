// 保证arr有序，才能用这个方法
fn exist(arr: Vec<i32>,num: i32) -> bool {
    if arr.len() == 0 {
        return false;
    }
    let mut l = 0;
    let mut r = arr.len() - 1;
    let mut m = 0;
    while l <= r {
        m = l + ((r - l) >> 1);
        if arr[m] == num {
            return true;
        }else if arr[m] > num {
            r = m - 1;
        }else {
            l = m + 1;
        }
    }
    return false;
}


// 峰值元素是指其值严格大于左右相邻值的元素
// 给你一个整数数组 nums，已知任何两个相邻的值都不相等
// 找到峰值元素并返回其索引
// 数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
// 你可以假设 nums[-1] = nums[n] = 无穷小
// 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。
//leetcode162
fn find_peak_element(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n == 1 {
        return 0;
    }
    if arr[0] > arr[1] {
        return 0;
    }
    if arr[n - 2] < arr[n - 1] {
        return n as i32 - 1;
    }
    let mut l = 1 ;
    let mut r = n  - 2;
    let mut m = 0;
    while l <= r {
        m = l + ((r - l) >> 1);
        if arr[m - 1] > arr[m] {
            r = m - 1;
        }else if arr[m] < arr[m + 1] {
            l = m + 1;
        }else {
            return m as i32;
        }
    }
    return -1;

}

fn main() {
    
}