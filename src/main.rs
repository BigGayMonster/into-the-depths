use console_error_panic_hook::set_once as set_panic_hook;
use mogwai::prelude::*;

const NS: &str = "http://www.w3.org/2000/svg";

mod event;
mod map;

fn world_view() -> ViewBuilder<HtmlElement> {
    builder! {
        <svg xmlns=NS width="100%" height="1000px">
            <rect xmlns=NS
                width="100%"
                height="100px"
                fill="lightskyblue"
            />
            <rect xmlns=NS
                y="100px"
                width="100%"
                height="1000px"
                fill="peru"
            />
            <rect xmlns=NS
                x="50%"
                y="120px"
                width="100px"
                height="50px"
                fill="sandybrown"
                rx="5"
                ry="5"
            />
        </svg>
    }
}

fn start_app() {
    console_log::init_with_level(log::Level::Trace).unwrap();

    let view = View::from(world_view());

    view.run().unwrap();
}
fn main() {
    set_panic_hook();
    start_app();
}
