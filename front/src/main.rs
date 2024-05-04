#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_spline::Spline;
use tracing::Level;
mod terminal;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        Spline { scene: "https://prod.spline.design/0Xa-BAzzB2XCS9WW/scene.splinecode" }
    }
}

