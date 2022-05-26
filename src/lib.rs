mod dice;

use rand::rngs::SmallRng;
use rand::SeedableRng;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn roll(s: &str) -> String {
    static mut RNG: Option<SmallRng> = None;
    let rng = unsafe { &mut RNG };
    if rng.is_none() {
        *rng = Some(SmallRng::from_entropy());
    }
    match dice::roll(s, rng.as_mut().unwrap()) {
        Ok(r) => format!("{}", r),
        Err(_) => "".to_string(),
    }
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
