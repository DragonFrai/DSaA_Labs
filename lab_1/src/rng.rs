pub struct Rng {
    a: u64,
    b: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { a: seed, b: !seed }
    }

    pub fn gen_i32_in(&mut self, min: i32, max: i32) -> i32 {
        let d = max - min + 1;
        let r = (self.gen_u64() % (d as u64)) as i32;
        r + min
    }

    pub fn gen_u64(&mut self) -> u64 {
        let mut t = self.a;
        let s = self.b;
        self.a = s;
        t ^= t << 23;
        t ^= t << 17;
        t ^= s ^ (s >> 26);
        self.b = t;
        t.wrapping_add(s)
    }

    pub fn gen_i64(&mut self) -> i64 {
        self.gen_u64() as i64
    }

    pub fn gen_u32(&mut self) -> u32 {
        self.gen_u64() as u32
    }

    pub fn gen_i32(&mut self) -> i32 {
        self.gen_u64() as i32
    }
}
