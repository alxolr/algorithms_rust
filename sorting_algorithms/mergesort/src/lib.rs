pub fn mergesort<T: PartialOrd + Copy>(arr: &mut [T]) {
    _mergesort(arr, 0, arr.len() - 1);
}

fn _mergesort<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) {
    if left < right {
        let mid = (left + right) / 2;
        _mergesort(arr, left, mid);
        _mergesort(arr, mid + 1, right);
        merge(arr, left, mid, right);
    }
}

fn merge<T: PartialOrd + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    let left_arr = &arr[left..=mid].to_owned();
    let right_arr = &arr[mid + 1..=right].to_owned();

    let mut i = 0;
    let mut j = 0;

    for k in left..=right {
        if i < left_arr.len() && j < right_arr.len() {
            if left_arr[i] < right_arr[j] {
                arr[k] = left_arr[i];
                i += 1;
            } else {
                arr[k] = right_arr[j];
                j += 1;
            }
        } else if i >= left_arr.len() {
            arr[k] = right_arr[j];
            j += 1;
        } else {
            arr[k] = left_arr[i];
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_function_works_as_expected() {
        let mut arr = vec![1, 4, 2, 7];
        merge(&mut arr, 0, 1, 3);

        assert_eq!(arr, &[1, 2, 4, 7]);
    }

    #[test]
    fn mergesort_properly_sorts_an_array() {
        let mut arr = vec![1, 4, 6, 48, 2, 7, 5, 11, 24, 32];
        mergesort(&mut arr);

        assert_eq!(arr, &[1, 2, 4, 5, 6, 7, 11, 24, 32, 48]);
    }
}
