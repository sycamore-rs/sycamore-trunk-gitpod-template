use sycamore::prelude::*;

#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    let name = ctx.create_signal(String::new());

    let displayed_name = move || {
        if name.get().is_empty() {
            "World".to_string()
        } else {
            name.get().as_ref().clone()
        }
    };

    view! { ctx,
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

    sycamore::render(|ctx| view! { ctx, App {} });
}
