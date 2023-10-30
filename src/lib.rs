pub mod data;
mod test;

pub fn run_some_task(data: Vec<[i32; 5]>) {
    running_methods::sequential_method(data);
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

        return value.lock().unwrap().iter().sum();
    }
}
