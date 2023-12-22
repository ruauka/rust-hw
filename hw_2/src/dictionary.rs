pub const VEC3_LEN: usize = 3;

pub type Pair = (i32, i32);
pub type Vec3 = [i32; VEC3_LEN];
pub type SignedCounter = isize;
pub type UnsignedCounter = usize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn default_pair() -> Pair {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_signed_counter() {
        assert_eq!(default_signed_counter(), 0);
    }

    #[test]
    fn test_default_unsigned_counter() {
        assert_eq!(default_unsigned_counter(), 0);
    }

    #[test]
    fn test_default_vec3() {
        let arr: [i32; VEC3_LEN] = [0; VEC3_LEN];
        assert_eq!(default_vec3(), arr);
    }

    #[test]
    fn test_default_pair() {
        assert_eq!(default_pair(), (0, 0));
    }
}
