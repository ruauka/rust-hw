/// 1. Принимает мутабельную ссылку на кортеж и bool значение.
/// - Если false, возвращает мутабельную ссылку на первый элемент кортежа.
/// - Если true, возвращает мутабельную ссылку на второй элемент кортежа.
pub fn func1(tuple: &mut (isize, isize), flag: bool) -> &mut isize {
    if flag {
        &mut tuple.0
    } else {
        &mut tuple.1
    }
}

/// 2. Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
pub fn func2(sl: &mut [isize], n: usize) -> &mut isize {
    &mut sl[n]
}

/// 3. Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
pub fn func3(sl: &[isize], n: usize) -> &isize {
    &sl[sl.len() - n - 1]
}

/// 4. Принимает слайс и число N. Возвращает два слайса с элементами:
/// - с нулевого по N-1;
/// - с N-го по последний;
pub fn func4(sl: &[isize], n: usize) -> (&[isize], &[isize]) {
    (&sl[..n + 1], &sl[n + 1..])
}

/// Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
pub fn func5(sl: &[isize]) -> [&[isize]; 4] {
    let mut res: [&[isize]; 4] = [&[]; 4];

    if sl.is_empty() {
        return res;
    }

    let left: &[isize] = &sl[..sl.len() / 2];
    let right: &[isize] = &sl[sl.len() / 2..];

    res[0] = &left[..left.len() / 2];
    res[1] = &left[left.len() / 2..];
    res[2] = &right[..right.len() / 2];
    res[3] = &right[right.len() / 2..];

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func1() {
        let mut tuple: (isize, isize) = (10, 20);

        let actual1: &mut isize = func1(&mut tuple, true);
        assert_eq!(*actual1, 10);

        let actual1: &mut isize = func1(&mut tuple, false);
        assert_eq!(*actual1, 20);
    }

    #[test]
    fn test_func2() {
        let mut arr: [isize; 5] = [10, 20, 30, 40, 50];
        let actual: &mut isize = func2(&mut arr, 3);

        assert_eq!(*actual, 40)
    }

    #[test]
    fn test_func3() {
        let arr: [isize; 5] = [10, 20, 30, 40, 50];
        let actual: &isize = func3(&arr, 3);

        assert_eq!(*actual, 20)
    }

    #[test]
    fn test_func4() {
        let arr: [isize; 5] = [10, 20, 30, 40, 50];
        let expected: (&[isize], &[isize]) = (&[10, 20, 30, 40], &[50]);

        let actual: (&[isize], &[isize]) = func4(&arr[..], 3);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_func5() {
        let arr1: [isize; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
        let expected1: [&[isize]; 4] = [&[10, 20], &[30, 40], &[50, 60], &[70, 80]];
        let actual1: [&[isize]; 4] = func5(&arr1);
        assert_eq!(actual1, expected1);

        let arr2 = [];
        let expected2: [&[isize]; 4] = [&[]; 4];
        let actual2: [&[isize]; 4] = func5(&arr2);
        assert_eq!(actual2, expected2);

        let arr3 = [10, 20, 30];
        let expected3: [&[isize]; 4] = [&[], &[10], &[20], &[30]];
        let actual3: [&[isize]; 4] = func5(&arr3);
        assert_eq!(actual3, expected3);

        let arr4 = [10, 20, 30, 40, 50];
        let expected4: [&[isize]; 4] = [&[10], &[20], &[30], &[40, 50]];
        let actual4: [&[isize]; 4] = func5(&arr4);
        assert_eq!(actual4, expected4)
    }
}
