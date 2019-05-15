extern crate cfg_if;

extern crate wasm_bindgen;
extern crate js_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let now = js_sys::Date::now();
    let now_date = js_sys::Date::new(&JsValue::from_f64(now));
    let p = document.create_element("p").unwrap();
    p.set_inner_html(&format!(
        "Hello World! It is {}:{}", 
        now_date.get_hours(),
        now_date.get_minutes(),
    ));
    document.body().unwrap().append_child(&p);

    Ok(())
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[wasm_bindgen]
pub struct Image {
    pixels: Vec<Color>,
}

#[wasm_bindgen]
impl Image {
    pub fn new() -> Image {
        let color1 = Color {
            red: 160,
            green: 0,
            blue: 0,
        };
        let color2 = Color {
            red: 0,
            green: 200,
            blue: 80,
        };
        let pixels = vec![color1, color2];
        Image {
            pixels
        }
    }

    pub fn pixels_ptr(&self) -> *const Color {
        self.pixels.as_ptr()
    }
}