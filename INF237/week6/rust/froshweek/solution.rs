use std::io::{self, BufRead};

fn merge_and_count_inversions(arr: &mut Vec<i32>, left_start: usize, middle: usize, right_end: usize) -> i32 {
    let size_left = middle - left_start + 1;
    let size_right = right_end - middle;


    let mut left = vec![0; size_left];
    let mut right = vec![0; size_right];

    for i in 0..size_left {
        left[i] = arr[left_start + i];
    }
    for j in 0..size_right {
        right[j] = arr[middle + 1 + j];
    }

    let (mut i, mut j, mut k) = (0, 0, left_start);
    let mut inversions = 0;


    while i < size_left && j < size_right {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;

            inversions += (size_left - i) as i32;
        }
        k += 1;
    }


    while i < size_left {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < size_right {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }

    inversions
}

fn merge_sort_and_count(arr: &mut Vec<i32>, left: usize, right: usize) -> i32 {
    let mut inversions = 0;
    if left < right {
        let middle = left + (right - left) / 2;

        inversions += merge_sort_and_count(arr, left, middle);
        inversions += merge_sort_and_count(arr, middle + 1, right);
        inversions += merge_and_count_inversions(arr, left, middle, right);
    }
    inversions
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let num: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();
    let mut to_be_sorted = Vec::with_capacity(num);

    for _ in 0..num {
        let new_num: i32 = iterator.next().unwrap().unwrap().trim().parse().unwrap();
        to_be_sorted.push(new_num);
    }

    let answer = merge_sort_and_count(&mut to_be_sorted, 0, num - 1);
    println!("{}", answer);
}
