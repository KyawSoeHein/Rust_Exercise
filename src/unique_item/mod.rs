fn find_unique_item<T: Ord>(list: Vec<T>) -> Option<Vec<T>> {
    if list.is_empty() {
        return None;
    }

    let mut result_list = Vec::new();

    for item in list {
        if !result_list.contains(&item) {
            result_list.push(item);
        }
    }

    Some(result_list)
}

#[cfg(test)]
mod tests {
    use crate::unique_item::find_unique_item;

    #[test]
    fn test_empty_list() {
        assert_eq!(find_unique_item::<i32>(vec![]), None)
    }

    #[test]
    fn test_duplicate_list() {
        assert_eq!(
            find_unique_item::<i32>(vec![1, 1, 2, 3, 8, 4]),
            Some(vec![1, 2, 3, 8, 4])
        )
    }

    #[test]
    fn test_list() {
        assert_eq!(
            find_unique_item::<i32>(vec![1, 2, 3, 8, 4]),
            Some(vec![1, 2, 3, 8, 4])
        )
    }

    #[test]
    fn test_list_order() {
        assert_eq!(
            find_unique_item::<i32>(vec![1, 3, 10, 8, 4]),
            Some(vec![1, 3, 10, 8, 4])
        )
    }
}
