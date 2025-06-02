mod xrng;
use xrng::Xrng;

pub struct OverworldBedrock {
    xr: Xrng,
}

impl crate::BedrockGenerator for OverworldBedrock {
    fn new(seed: u64) -> Self {
        //initialize a world + random generator
        //rng calculation
        let mut xr = Xrng::new();

        xr.create_xrng_seed(seed);
        xr.create_random_deriver();
        xr.create_random_string();
        xr.create_random_deriver();

        Self { xr }
    }

    fn is_bedrock(&self, x: i32, y: i32, z: i32) -> bool {
        //density calculation
        let density = crate::lerp_from_progress(y as f32, -64.0, -59.0, 1.0, 0.0);

        //copy xrng value
        let mut xr = self.xr.clone();
        xr.create_random(x, y, z);
        xr.next_float() < density
    }
}
