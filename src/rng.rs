use rand::{rngs::SmallRng, SeedableRng};



static mut rng : Option<SmallRng> = None;

pub fn get_rng() -> &'static mut SmallRng{
    unsafe {
        if rng.is_none() {
            rng = Some(SmallRng::seed_from_u64(1));
        }
        rng.as_mut().unwrap()
    }
}