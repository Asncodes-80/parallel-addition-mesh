pub mod data;

pub fn run_some_task(data: Vec<[i32; 5]>) {
    running_methods::parallel_method(data);
}

pub mod running_methods {
    use rayon::prelude::*;
    use std::sync::{Mutex, MutexGuard};

    pub fn sequential_method(data: Vec<[i32; 5]>) -> i32 {
        let mut a_column: Vec<i32> = Vec::new();
        let mut sum: i32 = 0;

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                sum += data[i][j];
            }
            a_column.push(sum);
            sum = 0;
        }

        for i in 0..a_column.len() {
            sum += a_column[i];
        }

        return sum;
    }

    pub fn parallel_method(data: Vec<[i32; 5]>) -> i32 {
        let value: Mutex<Vec<i32>> = Mutex::new(vec![]);

        data.par_iter().for_each(|row| {
            // It's parallel `a_column` list
            let v: i32 = row.par_iter().sum();
            let mut value: MutexGuard<'_, Vec<i32>> = value.lock().unwrap();
            value.push(v);
        });

        return value.lock().unwrap().par_iter().sum();
    }
}

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
