use crate::util;
use java_rand::Random;

pub fn is_bedrock(world_seed: u64, x: i32, y: i32, z: i32) -> bool {
    let mut state = Random::new(world_seed);

    //this is when we directly have to interface with the state
    let mut raw_state: u64 = Random::next_u64(&mut state) & ((1 << 48) - 1);

    
    if y < 0 || (y > 4 && y < 123) || y > 127 {
        return false;
    }
    if y < 5 {
        raw_state ^= util::text_hash_code("minecraft:bedrock_floor") as u64;
    }
    else {
        raw_state ^= util::text_hash_code("minecraft:bedrock_roof") as u64;
    }

    Random::set_seed(&mut state, raw_state);
    raw_state = Random::next_u64(&mut state) & ((1 << 48) - 1);
    raw_state ^= util::hash_code(x, y, z) as u64;

    Random::set_seed(&mut state, raw_state);

    //dbg!(&state);
    if y < 5 {
        let density = util::lerp_from_progress(y as f32, 0.0, 5.0, 1.0, 0.0);
        return Random::next_f32(&mut state) < density;
    }
    else {
        let density = util::lerp_from_progress(y as f32, 127.0, 127.0 - 5.0, 1.0, 0.0);
        return Random::next_f32(&mut state) > (1.0 - density);
    }
}