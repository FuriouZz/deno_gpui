use gpui::*;

mod element;
use element::*;

#[no_mangle]
extern "C" fn start(pointer: *mut u8, length: usize) {
    let element_string = unsafe { String::from_raw_parts(pointer, length, length) };
    let element: JSElement = serde_json::from_str(&element_string).unwrap();
    App::new().run(move |cx| {
        cx.open_window(WindowOptions::default(), |cx| cx.new_view(|_| element));
    });
}
