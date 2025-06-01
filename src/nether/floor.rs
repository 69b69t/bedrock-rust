use super::java_rand::Random;

#[derive(Clone)]
pub struct NetherBedrock {
    state: Random,
}

//implement floor here to make it a little more seperate
impl NetherBedrock {
    pub fn new(seed: u64) -> Self {
        //generate a state for the bedrock floor
        let mut state = Random::new(seed);
        state.state.0 = (state.next_u64() & ((1 << 48) - 1)) as i64;
        state.state.0 ^= crate::text_hash_code("minecraft:bedrock_floor") as i64;
        state.set_seed(state.state.0 as u64);
        state.state.0 = (state.next_u64() & ((1 << 48) - 1)) as i64;
        Self { state }
    }

    pub fn is_bedrock(&self, x: i32, y: i32, z: i32) -> bool {
        let density = crate::lerp_from_progress(y as f32, 0.0, 5.0, 1.0, 0.0);

        //make copy of state so we dont mess it up
        let mut temp_rng = self.state.clone();
        temp_rng.state.0 ^= crate::hash_code(x, y, z);
        temp_rng.set_seed(temp_rng.state.0 as u64);

        temp_rng.next_f64() < (density as f64)
    }
}
