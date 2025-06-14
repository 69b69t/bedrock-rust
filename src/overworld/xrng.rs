//cant find a suitable xoroshiro implementation

#[derive(Clone, Debug)]
pub struct Xrng {
    pub high: u64,
    pub low: u64,
}

impl Xrng {
    pub fn new() -> Self {
        Self { high: 0, low: 0 }
    }
    //these modify or use the Xrng state
    pub fn next_long(&mut self) -> u64 {
        let mut high = self.high;
        let low = self.low;

        //since we are rotating 17 bits left, really only the
        //seventeenth upper and beyond bits matter for modifying n after
        //the rotl, and low's upper 0 and beyond matter.
        //considering we are only using 24 bits of the upper state,
        //only bits 17 - (17 + 24) of high matter.
        //bits 0 to 24 and 17 to (17 + 24) of low matter.
        //there are large spikes of influence at 0 and 17 bits on low
        //and at 17 bits in on high.
        //all of these values are counting from MSB to LSB

        //really about 4 bits per each of these imfluence spikes matter to the state
        let n = rotl64(high.wrapping_add(low), 17).wrapping_add(low);
        high ^= low;
        self.high = rotl64(high, 28);
        self.low = rotl64(low, 49) ^ high ^ (high << 21);
        n
    }

    pub fn next(&mut self, bits: u8) -> u64 {
        self.next_long() >> (64 - bits)
    }

    pub fn next_float(&mut self) -> f32 {
        (self.next(24) as f32) / ((1 << 24) as f32)
    }

    pub fn create_xrng_seed(&mut self, seed: u64) {
        let l = seed ^ 0x6A09E667F3BCC909;
        let m = l.wrapping_add(0x9e3779b97f4a7c15);
        self.high = split_mix_64_int(m);
        self.low = split_mix_64_int(l);
    }

    pub fn create_random_deriver(&mut self) {
        let low = self.next_long();
        self.high = self.next_long();
        self.low = low;
    }

    pub fn create_random(&mut self, x: i32, y: i32, z: i32) {
        let l = crate::hash_code(x, y, z) as u64;
        self.low = l ^ self.low;
    }

    pub fn create_random_string(&mut self) {
        //typically this would do an md5 hash on minecraft:bedrock_floor
        //but since that never changes we will hardcode it in here
        //md5("minecraft:bedrock_floor") = bbf7928b7bf1d285c4dc7cf90e1b3b94
        self.high ^= 0xc4dc7cf90e1b3b94;
        self.low ^= 0xbbf7928b7bf1d285;
    }
}

fn rotl64(x: u64, b: u8) -> u64 {
    (x << b) | (x >> (64 - b))
}

fn split_mix_64_int(seed: u64) -> u64 {
    let mut seed = (seed ^ (seed >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    seed = (seed ^ (seed >> 27)).wrapping_mul(0x94D049BB133111EB);
    seed ^ (seed >> 31)
}