pub mod nether;
pub mod overworld;

pub trait BedrockGenerator {
    /// Returns a type that implements BedrockGenerator
    fn new(seed: u64) -> Self;

    /// Returns a bool given an x y and z position
    fn is_bedrock(&self, x: i32, y: i32, z: i32) -> bool;

    /// Returns a `Vec<bool>` of bedrock.
    /// It should be noted that the returned Vec contains no metadata,
    /// nor does it have indicies to help you use the return value.
    /// It is stored as a flattened (3d -> 1d) array in x-z-y order.
    /// (the x variable changes fastest, then z, then y)
    /// You can index a position with the formula:
    /// `bedrock[x][z][y] = bedrock[(x) + (z * x_size) * (y * x_size * z_size)]`
    /// It is written with the y layer last, because most of the time, one does not
    /// need more than one y layer. It is there nonetheless for those who need it.
    ///
    /// # Panics
    ///
    /// This function will panic if supplied with minimum bound values greater
    /// than their maximum counterpart.
    ///
    /// # Examples
    /// ```
    /// use bedrockgen::BedrockGenerator; //bring trait into scope
    /// use bedrockgen::nether::floor::NetherBedrock; //bring floor::NetherBedrock into scope
    ///
    /// //seed the generator with seed 0
    /// let bedrock_generator = NetherBedrock::new(0);
    ///
    /// //is block (0, 5, 0) on seed 0 bedrock?
    /// println!("{}", bedrock_generator.is_bedrock(0, 5, 0));
    ///
    /// //generate 4 bedrock layers
    /// let floor = bedrock_generator.generate_range(0, 0, 0, 10000, 5, 10000);
    /// ```


    fn generate_range(
        &self,
        x_min: i32,
        y_min: i32,
        z_min: i32,
        x_max: i32,
        y_max: i32,
        z_max: i32,
    ) -> Vec<bool> {
        //put some bedrock into a chunk of bool

        //first x, then z, then y for ordering within the vec.
        //i want y to be the last because people tend to only need one layer

        //bedrock[x][z][y] = bedrock[(x) + (z * x_size) * (y * x_size * z_size)]
        let mut bedrock: Vec<bool> = Vec::new();

        for y in y_min..y_max {
            for z in z_min..z_max {
                for x in x_min..x_max {
                    bedrock.push(self.is_bedrock(x, y, z));
                }
            }
        }
        bedrock
    }
}

fn lerp(delta: f32, start: f32, end: f32) -> f32 {
    start + delta * (end - start)
}

fn get_lerp_progress(value: f32, start: f32, end: f32) -> f32 {
    (value - start) / (end - start)
}

fn lerp_from_progress(
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

fn hash_code(x: i32, y: i32, z: i32) -> i64 {
    let mut i = x.wrapping_mul(3129871) as i64;
    i ^= (z as i64).wrapping_mul(116129781);
    i ^= y as i64;
    i = i
        .wrapping_mul(i)
        .wrapping_mul(42317861)
        .wrapping_add(i.wrapping_mul(11));
    i >> 16
}

fn text_hash_code(text: &str) -> i32 {
    let mut hash: i32 = 0;
    for character in text.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(character as i32);
    }
    hash
}
