use sycamore::prelude::*;

#[component]
fn App<G: Html>() -> View<G> {
    let name = create_signal(String::new());

    let displayed_name = move || {
        name.with(|name| {
            if name.is_empty() {
                "World".to_string()
            } else {
                name.clone()
            }
        })
    };

    view! {
        div {
            h1 {
                "Hello "
                (displayed_name())
                "!"
            }

            input(placeholder="What is your name?", bind:value=name)
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(App);
}
