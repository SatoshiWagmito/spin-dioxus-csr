use dioxus::prelude::*;
use wasm_bindgen::prelude::*;


// This external block is correctly defined for logging.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


// Correct start function for WebAssembly applications.
#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let window_exists = web_sys::window().is_some();
    if window_exists {
        log("This WebAssembly module is intended to be run in a web environment.");
        LaunchBuilder::new()
            .launch(app);
    } else {
        log("This WebAssembly module is not running in a web environment.");
    }
}


// tailwind - https://dioxuslabs.com/learn/0.5/cookbook/tailwind
// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("pkg/tailwind.css"));


fn app() -> Element {
    rsx! {
        div {
            class: "bg-white",
            div {
                class: "mx-auto max-w-7xl py-24 sm:px-6 sm:py-32 lg:px-8",
                div {
                    class: "relative isolate overflow-hidden bg-gray-900 px-6 pt-16 shadow-2xl sm:rounded-3xl sm:px-16 md:pt-24 lg:flex lg:gap-x-20 lg:px-24 lg:pt-0",
                    svg {
                        class: "absolute left-1/2 top-1/2 -z-10 h-[64rem] w-[64rem] -translate-y-1/2 [mask-image:radial-gradient(closest-side,white,transparent)] sm:left-full sm:-ml-80 lg:left-1/2 lg:ml-0 lg:-translate-x-1/2 lg:translate-y-0",
                        view_box: "0 0 1024 1024",
                        //aria_hidden: "true",
                        circle {
                            fill: "url(#759c1415-0410-454c-8f7c-9a820de03641)",
                            fill_opacity: "0.7",
                            cy: "512",
                            r: "512",
                            cx: "512",
                        }
                        defs {
                            radialGradient {
                                id: "759c1415-0410-454c-8f7c-9a820de03641",
                                stop {
                                    stop_color: "#7775D6"
                                }
                                stop {
                                    offset: "1",
                                    stop_color: "#E935C1"
                                }
                            }
                        }
                    }

                    div {
                        class: "mx-auto max-w-md text-center lg:mx-0 lg:flex-auto lg:py-32 lg:text-left",
                        h2 {
                            class: "text-3xl font-bold tracking-tight text-white sm:text-4xl",
                            "Fermyon Spin + Dioxus"
                            br {

                            }
                            "Client Side Rendering (CSR)"
                        }
                        p {
                            class: "mt-6 text-lg leading-8 text-gray-300",
                            "Ac euismod vel sit maecenas id pellentesque eu sed consectetur. Malesuada adipiscing sagittis vel nulla."
                        }
                        div {
                            class: "mt-10 flex items-center justify-center gap-x-6 lg:justify-start",
                            a {
                                class: "rounded-md bg-white px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-gray-100 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white",
                                href: "#","Get started"
                            }
                            a {
                                class: "text-sm font-semibold leading-6 text-white",
                                href: "#","Learn more"
                                span {
                                    "aria-hidden": "true","→"
                                }
                            }
                        }
                    }
                    div {
                        class: "relative mt-16 h-80 lg:mt-8",
                        img {
                            class: "absolute left-0 top-0 w-[57rem] max-w-none rounded-md bg-white/5 ring-1 ring-white/10",
                            alt: "App screenshot",
                            height: "1080",
                            src: "https://tailwindui.com/img/component-images/dark-project-app-screenshot.png",
                            width: "1824",
                        }
                    }
                }
            }
        }
    }
}
