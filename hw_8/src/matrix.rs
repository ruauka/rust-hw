/// общий трейт для типа матрицы (T)
pub trait Number:
    std::ops::Add<Output = Self> + std::ops::Mul<Output = Self> + std::fmt::Debug + Sized + Copy
{
    /// нулевое дефолтное значение для дженерика
    fn default() -> Self;
    /// значение для дженерика равное 1 для умножкния
    fn one() -> Self;
    /// проверка на дефолтное значение
    fn is_zero(&self) -> bool;
}

/// имплементация для f64 для типа элементов матрицы
impl Number for f64 {
    fn default() -> Self {
        0_f64
    }
    /// значение для дженерика равное 1 для умножкния
    fn one() -> Self {
        1_f64
    }
    /// проверка на дефолтное значение
    fn is_zero(&self) -> bool {
        *self == 0_f64
    }
}

/// структура матрицы
#[derive(Debug)]
pub struct Matrix<T: Number, const N: usize> {
    pub data: [T; N],
}

/// перегрузка метода add()
/// имплементация std::ops::Add<T> -  для складывания каждого элемента матрицы и числа через знак +
impl<T: Number, const N: usize> std::ops::Add<T> for Matrix<T, N> {
    type Output = Self;

    fn add(self, num: T) -> Matrix<T, N> {
        let mut res: Matrix<T, N> = Matrix {
            data: [T::default(); N],
        };

        for i in 0..N {
            res.data[i] = self.data[i] + num
        }

        res
    }
}

/// перегрузка метода mul()
/// имплементация std::ops::Mul<T> - для перемножения каждого элемента матрицы и числа через знак *
impl<T: Number, const N: usize> std::ops::Mul<T> for Matrix<T, N> {
    type Output = Self;

    fn mul(self, num: T) -> Matrix<T, N> {
        let mut res: Matrix<T, N> = Matrix {
            data: [T::default(); N],
        };

        for i in 0..N {
            res.data[i] = self.data[i] * num
        }

        res
    }
}
