mod counter;
mod dictionary;

use crate::dictionary::{default_vec3, Pair, Vec3, VEC3_LEN};

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_vector_sum() {
        let arr1: [i32; VEC3_LEN] = [1, 2, 3];
        let arr2: [i32; VEC3_LEN] = [4, 5, 6];

        assert_eq!(vec3_vector_sum(arr1, arr2), [5, 7, 9]);
    }

    #[test]
    fn test_pair_vector_sum() {
        let tup1: (i32, i32) = (1, 2);
        let tup2: (i32, i32) = (3, 4);

        assert_eq!(pair_vector_sum(tup1, tup2), (4, 6));
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let arr1: [i32; VEC3_LEN] = [1, 2, 3];
        let arr2: [i32; VEC3_LEN] = [4, 5, 6];

        assert_eq!(vec3_scalar_sum(arr1, arr2), 21);
    }

    #[test]
    fn test_pair_scalar_sum() {
        let tup1: (i32, i32) = (1, 2);
        let tup2: (i32, i32) = (3, 4);

        assert_eq!(pair_scalar_sum(tup1, tup2), 10);
    }
}
