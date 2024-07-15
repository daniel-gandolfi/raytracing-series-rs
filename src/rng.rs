use rand::{rngs::SmallRng, SeedableRng};

static mut RNG: Option<SmallRng> = None;

pub fn get_rng() -> &'static mut SmallRng {
    unsafe {
        if RNG.is_none() {
            RNG = Some(SmallRng::seed_from_u64(1));
        }
        RNG.as_mut().unwrap()
    }
}
