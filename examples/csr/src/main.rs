use leptos::{event_target_value, mount_to_body, view, SignalSet};
use leptos_qr::QrCode;

fn main() {
    console_error_panic_hook::set_once();

    let (qr_text, set_qr_text) = leptos::create_signal("Hello, World!".to_string());

    mount_to_body(move || {
        view! {
            <div style="width: 200px; color: red;">
                <QrCode
                    data=qr_text
                    ecl=leptos_qr::ECL::Q
                    shape=leptos_qr::Shape::Circle
                    fg_color="#111111"
                    bg_color="#dddddd"
                />
            </div>
            <input
                type="text"
                value="Hello, World!"
                on:input=move |e| {
                    set_qr_text.set(event_target_value(&e));
                }
                prop:value=qr_text
            />
        }
    })
}