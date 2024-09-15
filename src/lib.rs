use leptos::*;
use fast_qr::QRBuilder;
use fast_qr::convert::{Builder};
use fast_qr::convert::svg::SvgBuilder;

pub use fast_qr::convert::{Color, Shape};
pub use fast_qr::ECL;


#[component]
pub fn QrCode(
    #[prop(into)] data: MaybeSignal<String>,
    #[prop(into, optional)] fg_color: Option<String>,
    #[prop(into, optional)] bg_color: Option<String>,
    #[prop(into, optional)] shape: Option<Shape>,
    #[prop(into, optional)] ecl: Option<ECL>,
) -> impl IntoView {

    let fg_color_inner = fg_color.clone();
    let bg_color_inner = bg_color.clone();
    let qr_svg = move || {
        let qrcode = QRBuilder::new(data.get())
            .ecl(ecl.unwrap_or(ECL::M))
            .build()
            .unwrap();

        let svg = SvgBuilder::default()
            .shape(shape.unwrap_or(Shape::Square))
            .background_color(Color(bg_color_inner.clone().unwrap_or_else( || "#FFFFFF".to_string())))
            .module_color(Color(fg_color_inner.clone().unwrap_or_else( || "#000000".to_string())))
            .to_str(&qrcode);

        svg
    };

    view! {
        <div
            inner_html=qr_svg
        />
    }
}

