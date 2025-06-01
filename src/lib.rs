pub mod nether;
pub mod overworld;

fn lerp(delta: f32, start: f32, end: f32) -> f32 {
    start + delta * (end - start)
}

fn get_lerp_progress(value: f32, start: f32, end: f32) -> f32 {
    (value - start) / (end - start)
}

pub fn lerp_from_progress(
    lerp_value: f32,
    lerp_start: f32,
    lerp_end: f32,
    start: f32,
    end: f32,
) -> f32 {
    lerp(
        get_lerp_progress(lerp_value, lerp_start, lerp_end),
        start,
        end,
    )
}

pub fn hash_code(x: i32, y: i32, z: i32) -> i64 {
    let mut i = x.wrapping_mul(3129871) as i64;
    i ^= (z as i64).wrapping_mul(116129781);
    i ^= y as i64;
    i = i
        .wrapping_mul(i)
        .wrapping_mul(42317861)
        .wrapping_add(i.wrapping_mul(11));
    i >> 16
}

pub fn text_hash_code(text: &str) -> i32 {
    let mut hash: i32 = 0;
    for character in text.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(character as i32);
    }
    hash
}
