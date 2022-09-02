mod js;

use js::{
    add_bang, list_it, lodash_concat, log, log_each, log_number_list, say_hello, say_hello_name,
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let onclick = Callback::from(|_| {
        say_hello();
        say_hello_name("Twitch Chat");
        let name_with_bang = add_bang("HELLO");

        log(&name_with_bang);

        let numbers = list_it(15.0, 32.0);
        log_number_list(numbers.clone());
        log_each(numbers.clone());

        let many_numbers = lodash_concat(&vec![64.4, 12.2], &vec![42.1, 32.9], &vec![36.9, 12.1]);
        log_number_list(many_numbers);
    });

    html! {
        <>
            <h1>{"Can we run JS from Rust"}</h1>
            <div>
                <button {onclick}>{"say hello"}</button>
            </div>
        </>
    }
}
