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