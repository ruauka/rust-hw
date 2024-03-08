#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

mod matrix;

use matrix::{Matrix, Number};
use std::ops::Add;

/// структура набора матриц
#[derive(Debug)]
struct MtxArray<'a, T: Number, const N: usize> {
    mtxs: Vec<&'a Matrix<T, N>>,
}

impl<'a, T: Number + for<'b> std::iter::Sum<&'b T>, const N: usize> MtxArray<'a, T, N> {
    /// получить матрицу по индексу
    pub fn get_mtx(&self, idx: usize) -> &Matrix<T, N> {
        self.mtxs[idx]
    }

    /// сложить все элементы всех матриц
    pub fn sum_all(&self) -> T {
        let mut total: T = T::default();

        for mtx in self.mtxs.iter() {
            total = total + mtx.data.iter().sum();
        }

        total
    }

    /// перемножить все элементы всех матриц
    pub fn mult_all(&self) -> T {
        let mut total: T = T::default();

        for mtx in self.mtxs.iter() {
            for num in mtx.data {
                if total.is_zero() {
                    total = T::one()
                }
                total = total * num
            }
        }

        total
    }
}

fn main() {
    let mtx1 = Matrix {
        data: [1_f64, 2_f64, 3_f64],
    };
    // let num1: f64 = 100_f64;
    // let res1: Matrix<f64, 3> = mtx1 + num1;
    // println!("{:?}", res1);

    let mtx2 = Matrix {
        data: [10_f64, 20_f64, 30_f64],
    };
    // let num2: f64 = 5_f64;
    // let res2: Matrix<f64, 3> = mtx2 * num2;
    // println!("{:?}", res2);

    let mtxs = MtxArray {
        mtxs: vec![&mtx1, &mtx2],
    };
    let res = mtxs.get_mtx(1);
    println!("{:?}", res); // Matrix { data: [10.0, 20.0, 30.0] }

    let sum_res = mtxs.sum_all();
    println!("{:?}", sum_res); // 66.0

    let mult_res = mtxs.mult_all();
    println!("{:?}", mult_res); // 66.0
}
