#[cfg(test)]
mod test {
    use crate::running_methods;

    #[test]
    fn test_parallel_method() {
        let data = vec![
            [6, -4, 19, 2, 0],
            [-9, 0, 3, -5, 0],
            [10, -3, -8, 1, 0],
            [7, -2, 4, 5, 0],
        ];
        let parallel_result: i32 = running_methods::parallel_method(data);
        assert_eq!(parallel_result, 26);
    }

    #[test]
    fn test_sequential_method() {
        let data = vec![
            [6, -4, 19, 2, 0],
            [-9, 0, 3, -5, 0],
            [10, -3, -8, 1, 0],
            [7, -2, 4, 5, 0],
        ];

        let sequential_result: i32 = running_methods::sequential_method(data);
        assert_eq!(sequential_result, 26);
    }
}
