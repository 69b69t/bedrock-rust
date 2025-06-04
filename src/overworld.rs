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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BedrockGenerator;

    #[test]
    fn test_bedrock_generation() {
        //make a new BedrockGenerator
        let bedrock_generator = OverworldBedrock::new(0);

        //this should be enough that even the most similar of bedrock patterns are not equal.
        //a better way to do this would be to feed a hasher the boolean values
        //and check the output, because right now we are only counting the number of bedrock,
        //which is falliable
        let bedrock_pattern = bedrock_generator.generate_range(0, -64, 0, 10000, -59, 10000);
        let mut counter = 0;
        for bedrock in bedrock_pattern {
            if bedrock {
                counter += 1;
            }
        }
        println!("{counter} bedrock generated");
        assert_eq!(counter, 300009082);
    }
}