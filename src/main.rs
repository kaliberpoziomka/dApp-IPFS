/// Import Yew
use yew::prelude::*;


#[derive(Properties, PartialEq)]
struct TodoProps {
    state: UseStateHandle<u64>,
}

/// A Counter functional component (like in React)
#[function_component(Counter)]
fn counter(props: &TodoProps) -> Html {

    let increment = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(*state + 1))
    }; 

    let decrement = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    let reset = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(0))
    };

    // The component written in HTML, thanks to html! macro
    html! {
        <div class="uk-position-center uk-text-center">
            <button 
                onclick={increment}
                class="uk-button uk-button-primary uk-button-large"
            >
                { "+1" }
            </button>
            <button
                onclick={decrement}
                class="uk-button uk-button-primary uk-button-large"
            >
                { "-1" }
            </button>
            <button
                onclick={reset}
                class="uk-button uk-button-primary uk-button-large">
                { "reset" }
            </button>
            <p>{ *props.state }</p>
        </div>
    }
}

/// App's initial state
#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| 0 as u64); // this: || 0  is closure, i.e anonymous function. We also cast it as 64 (by default it is i32)

    html! {
        <Counter {state} />
    }
}

/// Start the Yew app
fn main() {
    yew::start_app::<App>();
}