mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Document, KeyboardEvent, Window};

#[wasm_bindgen]
extern "C" {
    // This function is already provided in your code
    // It's an example of how to call JavaScript functions from Rust
    fn alert(s: &str);
}

// Utility function to obtain the window object
fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

// Utility function to obtain the document object
fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Set up the keypress event listener
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        console::log_1(&format!("Key pressed: {}", event.key()).into());
    }) as Box<dyn FnMut(_)>);

    document().add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Prevent the closure from being garbage collected

    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasmdemo!");
}
