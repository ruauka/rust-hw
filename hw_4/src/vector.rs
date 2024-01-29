pub const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];

pub struct Vector {
    vec_a: Vec3,
    vec_b: Vec3,
}

impl Vector {
    pub fn new(vec_a: Vec3, vec_b: Vec3) -> Self {
        Self { vec_a, vec_b }
    }

    pub fn default_vec3(&self) -> Vec3 {
        [0; 3]
    }

    pub fn vec3_vector_sum(&self) -> Vec3 {
        let mut c: Vec3 = self.default_vec3();
        for i in 0..3 {
            c[i] = self.vec_a[i] + self.vec_b[i];
        }
        c
    }

    pub fn vec3_scalar_sum(&self) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += self.vec_a[i] + self.vec_b[i];
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec3() {
        let arr: [i32; VEC3_LEN] = [0; VEC3_LEN];
        let vector: Vector = Vector::new(arr, arr);

        assert_eq!(vector.default_vec3(), arr);
    }

    #[test]
    fn test_vec3_vector_sum() {
        let arr1: [i32; VEC3_LEN] = [1, 2, 3];
        let arr2: [i32; VEC3_LEN] = [4, 5, 6];

        let vector: Vector = Vector::new(arr1, arr2);

        assert_eq!(vector.vec3_vector_sum(), [5, 7, 9]);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let arr1: [i32; VEC3_LEN] = [1, 2, 3];
        let arr2: [i32; VEC3_LEN] = [4, 5, 6];

        let vector: Vector = Vector::new(arr1, arr2);

        assert_eq!(vector.vec3_scalar_sum(), 21);
    }
}
