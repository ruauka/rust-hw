/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число

    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/
pub fn fizzbuzz(num: u32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        return "FizzBuzz".to_string();
    };
    if num % 3 == 0 {
        return "Fizz".to_string();
    };
    if num % 5 == 0 {
        return "Buzz".to_string();
    };

    num.to_string()
}

// pub fn fizzbuzz_list(n: u32) -> Vec<String> {
//     // не понял, что надо сделать
// }

/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при это цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_length(num: u32) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}

fn num_multiple(str_vec: &Vec<char>) -> u32 {
    let mut res: u32 = 1;

    for val in str_vec {
        if val == &'0' {
            continue;
        }
        res *= val.to_string().parse::<u32>().unwrap()
    }

    res
}

fn make_char_vec(num: u32) -> Vec<char> {
    let mut res: Vec<char> = Vec::new();

    for val in num.to_string().chars() {
        res.push(val)
    }

    res
}

pub fn digit_product(n: u32) -> u8 {
    if n == 0 {
        return 0;
    }

    let mut char_vec: Vec<char> = make_char_vec(n);
    let mut result: u32 = num_multiple(&char_vec);

    loop {
        if digit_length(result) == 1 {
            break;
        }

        char_vec = make_char_vec(result);
        result = num_multiple(&char_vec);
    }

    result as u8
}

/*
    Последовательностью Фибоначчи называется последовательность чисел,
    которая удовлетворяет следующим условиям:
    - элемент последовательности с индексом 0 - число 0
    - элемент с индексом 1 - число 1
    - каждый последующий элемент равен сумме двух предыдущих.

    0, 1, 1, 2, 3, 5, 8, 13, 21 ...

    Написать функцию, которая вычислит элемент последовательности с индексом n.

    * Написать вторую функцию, которая вернёт последовательность Фибонначи
      от первого элемента до n-ого. Написать тесты
*/
pub fn fib(n: i64) -> i64 {
    if n < 2 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

/*
    Дана строка, состоящая только из цифровых символов. В данной строке
    есть одна цифра, которая не повторяется. Написать функцию,
    которая найдёт эту цифру и вернёт её.

    * Написать похожую функцию, но только на этот раз в данной строке
    могут присутствовать любые символы, а уникальная цифра может отсутствовать.
    Но если присутсвует, то не больше одной. Написать тесты.
*/

pub fn uniq_digit(s: &str) -> u8 {
    let mut vc: Vec<char> = Vec::new();

    for val in s.chars() {
        vc.push(val)
    }

    let mut counter: u8 = u8::default();
    let mut res: char = char::default();

    // вопрос, цикл забирает вектор во владение???
    for i in &vc {
        for j in &vc { // и что бы его тут использовать требуется указатель, так?
            if i == j {
                counter += 1
            }
        }
        if counter == 1 {
            res = *i
        }
        counter = 0;
    }

    // 0x30 - 48
    res as u8 - 0x30
}

/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.

    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/
pub fn missing_num(nums: &[i32]) -> i32 {
    let mut res: i32 = i32::default();

    let mut sorted_arr: Vec<i32> = Vec::new();
    for val in nums {
        sorted_arr.push(*val)
    }

    sorted_arr.sort();

    for (mut idx, val) in sorted_arr.iter().enumerate() {
        idx += sorted_arr[0] as usize;

        if idx == *val as usize {
            continue;
        }

        res = idx as i32;
        break;
    }

    res
}

/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/
pub fn validate_paren(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for bracket in s.chars() {
        if bracket == '{' || bracket == '[' || bracket == '(' {
            stack.push(bracket);
            continue;
        }
        if bracket == '}' || bracket == ']' || bracket == ')' {
            if stack.is_empty() {
                return false;
            }

            let last: char = stack[stack.len() - 1];

            if (bracket == '}' && last != '{')
                || (bracket == ']' && last != '[')
                || (bracket == ')' && last != '(')
            {
                return false;
            }

            stack.pop();
        }
    }

    stack.is_empty()
}
