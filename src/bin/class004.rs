// // 数组中交换i和j位置的数
fn swap(arr:&mut[i32],i:usize,j:usize){
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

// // 选择排序
fn selection_sort(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }
    
    for i in 0..arr.len() - 1 {
        let mut min_index = i; // Start with the first element
        
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j; // Update the index of the minimum element
            }
        }
        
        swap(arr, i, min_index); // Swap the found minimum element with the first element
    }
}

// // 冒泡排序
fn bubble_sort(arr: &mut [i32]){
    if arr.len() < 2 {
        return;
    }
    for i in (1..arr.len()).rev() {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                swap(arr, j + 1, j);
            }
        }
    }
}

// // 插入排序
fn insertion_sort(arr: &mut [i32]){
    if arr.len() < 2 {
        return;
    }
    for i in 1..arr.len() {
        for j in (0..i).rev() {
            if arr[j] > arr[j + 1] {
                swap(arr, j + 1, j);
            }else {
                break;
            }
        }
    }
}

fn main(){
    let mut arr1 = [1,3,5,2,-1,8,2];
    println!("{:?}",arr1);
    insertion_sort(&mut arr1);
    println!("{:?}",arr1);
}