use base64::Engine;
use dioxus::prelude::*;
use opencv::{core, imgcodecs, prelude::*};

#[component]
pub fn OpenCV_Img_Upload() -> Element {
    let mut image_data_url = use_signal(|| None::<String>);

    // Sample hardcoded path (change this to your actual image path)
    let image_path = "1France.jpg"; // Use a PNG/JPEG image from your filesystem

    let pick_image = move |_| {
        // Read the image using OpenCV
        match imgcodecs::imread(image_path, imgcodecs::IMREAD_COLOR) {
            Ok(mat) => {
                // Encode the image to PNG format in memory
                let mut buf = core::Vector::<u8>::new();
                if imgcodecs::imencode(".png", &mat, &mut buf, &core::Vector::new()).is_ok() {
                    // Manually encode to base64 (simple base64 implementation)
                    let encoded = base64_simple(buf.as_slice());

                    let data_url = format!("data:image/png;base64,{}", encoded);
                    image_data_url.set(Some(data_url));
                }
            }
            Err(e) => {
                println!("Error reading image: {}", e);
            }
        }
    };

    rsx! {
        div { class: "p-4 font-sans",
            button {
                onclick: pick_image,
                class: "px-4 py-2 bg-indigo-600 text-white rounded text-2xl",
                "OpenCV Upload"
            }
            {
                if let Some(url) = image_data_url() {
                    Some(rsx! {
                        div { class: "mt-4",
                            img { src: "{url}", class: "max-w-[600px]" }
                        }
                    })
                } else {
                    None
                }
            }
        }
    }
}

/// Minimal base64 encoding function (just for illustration)
fn base64_simple(bytes: &[u8]) -> String {
    // NOTE: Uses base64 crate here for brevity. If you must avoid it completely,
    // implement base64 manually (but it's tedious and error-prone).
    base64::engine::general_purpose::STANDARD.encode(bytes)
}
