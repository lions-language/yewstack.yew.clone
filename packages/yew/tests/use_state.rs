mod common;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::functional::{
    use_effect_with_deps, use_state, use_state_eq, FunctionComponent, FunctionProvider,
};
use yew::{html, HtmlResult};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn use_state_works() {
    struct UseStateFunction {}
    impl FunctionProvider for UseStateFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> HtmlResult {
            let counter = use_state(|| 0);
            if *counter < 5 {
                counter.set(*counter + 1)
            }
            return Ok(html! {
                <div>
                    {"Test Output: "}
                    <div id="result">{*counter}</div>
                    {"\n"}
                </div>
            });
        }
    }
    type UseComponent = FunctionComponent<UseStateFunction>;
    yew::start_app_in_element::<UseComponent>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    );
    let result = obtain_result();
    assert_eq!(result.as_str(), "5");
}

#[wasm_bindgen_test]
fn multiple_use_state_setters() {
    struct UseStateFunction {}
    impl FunctionProvider for UseStateFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> HtmlResult {
            let counter = use_state(|| 0);
            let counter_clone = counter.clone();
            use_effect_with_deps(
                move |_| {
                    // 1st location
                    counter_clone.set(*counter_clone + 1);
                    || {}
                },
                (),
            );
            let another_scope = {
                let counter = counter.clone();
                move || {
                    if *counter < 11 {
                        // 2nd location
                        counter.set(*counter + 10)
                    }
                }
            };
            another_scope();
            Ok(html! {
                <div>
                    { "Test Output: " }
                    // expected output
                    <div id="result">{ *counter }</div>
                    { "\n" }
                </div>
            })
        }
    }
    type UseComponent = FunctionComponent<UseStateFunction>;
    yew::start_app_in_element::<UseComponent>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    );
    let result = obtain_result();
    assert_eq!(result.as_str(), "11");
}

#[wasm_bindgen_test]
fn use_state_eq_works() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct UseStateFunction {}

    impl FunctionProvider for UseStateFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> HtmlResult {
            RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
            let counter = use_state_eq(|| 0);
            counter.set(1);

            Ok(html! {
                <div>
                    {"Test Output: "}
                    <div id="result">{*counter}</div>
                    {"\n"}
                </div>
            })
        }
    }
    type UseComponent = FunctionComponent<UseStateFunction>;
    yew::start_app_in_element::<UseComponent>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    );
    let result = obtain_result();
    assert_eq!(result.as_str(), "1");
    assert_eq!(RENDER_COUNT.load(Ordering::Relaxed), 2);
}
