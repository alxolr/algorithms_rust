pub fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

pub fn _quicksort<T: Ord>(arr: &mut [T], left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, 0, right);

        _quicksort(arr, left, partition_idx - 1);
        _quicksort(arr, partition_idx + 1, right);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: isize, right: isize) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partitioning_method() {
        let mut arr = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];
        let len = (arr.len() - 1) as isize;

        let partition_idx = partition(&mut arr, 0, len);
        assert_eq!(partition_idx, 7);
    }

    #[test]
    fn test_quicksort_sorts_properly() {
        let mut arr = vec![13, 19, 9, 5, 12, 8, 7, 4, 21, 2, 6, 11];

        quicksort(&mut arr);
        assert_eq!(arr, &[2, 4, 5, 6, 7, 8, 9, 11, 12, 13, 19, 21]);
    }
}
