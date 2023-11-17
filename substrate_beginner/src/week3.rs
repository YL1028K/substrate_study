use std::cmp::Ordering;

pub fn bubble_sort_i32(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    let mut i: usize = 0;
    while i < nums.len() {
        let mut j: usize = 1;
        while j < nums.len() {
            if nums[j] < nums[j - 1] {
                nums.swap(j, j - 1);
            }
            j += 1;
        }
        i += 1;
    }
    return;
}

pub fn bubble_sort<T: std::cmp::PartialOrd>(nums: &mut [T]) {
    if nums.len() < 2 {
        return;
    }
    let mut i: usize = 0;
    while i < nums.len() {
        let mut j: usize = 1;
        while j < nums.len() {
            match nums[j].partial_cmp(&nums[j - 1]) {
                Some(Ordering::Less) => nums.swap(j, j - 1),
                _ => (),
            }
            j += 1;
        }
        i += 1;
    }
    return;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_i32() {
        let mut nums: [i32; 6] = [-10, 2, 20, -3, 1, 10];
        let sorted = [-10, -3, 1, 2, 10, 20];
        bubble_sort_i32(&mut nums);
        assert_eq!(nums, sorted);
    }

    #[test]
    fn test_sort() {
        let mut nums: [i32; 6] = [-10, 2, 20, -3, 1, 10];
        let sorted: [i32; 6] = [-10, -3, 1, 2, 10, 20];
        bubble_sort(&mut nums);
        assert_eq!(nums, sorted);

        let mut strs: [String; 4] = [
            String::from("d"),
            String::from("z"),
            String::from("c"),
            String::from("q"),
        ];
        let sorted_strs: [String; 4] = [
            String::from("c"),
            String::from("d"),
            String::from("q"),
            String::from("z"),
        ];
        bubble_sort(&mut strs);
        assert_eq!(strs, sorted_strs);
    }
}
