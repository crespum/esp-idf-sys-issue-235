use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Changing the loop to run 2+ times causes an Illegal instruction (with esp-idf-sys = 0.33.2)
    for i in 0..2 {
        let n = (50.1 + (i as f32) * 2. % 100.1) * 7.1;
        println!("Number {n}");
    }
}
