use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus::LaunchBuilder;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod rfd_img_upload;
use crate::rfd_img_upload::RFD_Image_Upload;

mod opencv_img_upload;
use crate::opencv_img_upload::OpenCV_Img_Upload;
fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::default()
                .with_window(WindowBuilder::new().with_title("My App"))
                .with_menu(None),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Stylesheet { href: MAIN_CSS }
        // document::Stylesheet { href: TAILWIND_CSS }

        // style { "{include_str!(\"../assets/main.css\")}" }
        style { "{include_str!(\"../assets/tailwind.css\")}" }

        RFD_Image_Upload {}
        // OpenCV_Img_Upload {}

    }
}
