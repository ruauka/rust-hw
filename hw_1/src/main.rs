// функция double_int32 принимает 32-х битное целое беззнаковое число и
// возвращает 32-х битное целое беззнаковое число, равное удвоенному входному
fn double_int32(foo: u32) -> u32 {
    foo
}

// функция double_int64 принимает 32-х битное целое беззнаковое число и
// возвращает 64-х битное целое беззнаковое число, равное удвоенному входному.
fn double_int64(foo: u32) -> u64 {
    foo as u64 * 2
}

// функция double_float32 принимает 32-х битное число с плавающей точкой и
// возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.
fn double_float32(foo: f32) -> f32 {
    foo * 2.0
}

// функция double_float64 принимает 32-х битное число с плавающей точкой и
// возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
fn double_float64(foo: f32) -> f64 {
    foo as f64 * 2.0
}

// ункция int_plus_float_to_float принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой.
// Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
fn int_plus_float_to_float(foo: u32, bar: f32) -> f64 {
    foo as f64 + bar as f64
}

// функция int_plus_float_to_int принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой.
// Возвращает 64-х битное целое беззнаковое число, равное сумме входных
fn int_plus_float_to_int(foo: u32, bar: f32) -> u64 {
    foo as u64 + bar as u64
}

// функция tuple_sum принимает кортеж из двух целых чисел.
// Возвращает целое число, равное сумме чисел во входном кортеже.
fn tuple_sum(foo: (isize, isize)) -> isize {
    foo.0 + foo.1
}

// функция array_sum принимает массив из трёх целых чисел.
// Возвращает целое число, равное сумме чисел во входном массиве.
fn array_sum(foo: [isize; 3]) -> isize {
    let mut res: isize = 0;

    for val in foo.iter() {
        res += val
    };

    res
}


fn main() {
    let res: u32 = double_int32(1);
    println!("1. double_int32 res: {}", res);

    let res: u64 = double_int64(2);
    println!("2. double_int64 res: {}", res);

    let res: f32 = double_float32(1.2345);
    println!("3. double_float32 res: {}", res);

    let res: f64 = double_float64(2.3456);
    println!("4. double_float64 res: {}", res);

    let res: f64 = int_plus_float_to_float(10, 2.3456);
    println!("5. int_plus_float_to_float res: {}", res);

    let res: u64 = int_plus_float_to_int(10, 2.3456);
    println!("6. int_plus_float_to_int res: {}", res);

    let res: isize = tuple_sum((10, 20));
    println!("7. tuple_sum res: {}", res);

    let res: isize = array_sum([1, 2, 3]);
    println!("8. array_sum res: {}", res);
}
