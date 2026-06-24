use yew::prelude::*;
use crate::models::FAQ;

#[function_component(FAQSection)]
pub fn faq_section() -> Html {
    let open = use_state(|| None::<usize>);

    html! {
        <section class="section section-faq">
            <div class="container">
                <h2 class="section-title">{ "Dúvidas frequentes" }</h2>
                <p class="section-subtitle">{ "Tire suas dúvidas e venha com tranquilidade" }</p>
                <div class="faq-list">
                    { FAQ.iter().enumerate().map(|(i, item)| {
                        let is_open = *open == Some(i);
                        let toggle = {
                            let open = open.clone();
                            Callback::from(move |_| {
                                if *open == Some(i) {
                                    open.set(None);
                                } else {
                                    open.set(Some(i));
                                }
                            })
                        };
                        let answer_id = format!("faq-answer-{}", i);
                        html! {
                            <div class={classes!("faq-item", is_open.then(|| "faq-open"))}>
                                <button class="faq-question" onclick={toggle}
                                    aria-expanded={is_open.to_string()}
                                    aria-controls={answer_id.clone()}>
                                    <span>{ item.question }</span>
                                    <span class="faq-arrow">{ if is_open { "−" } else { "+" } }</span>
                                </button>
                                if is_open {
                                    <div class="faq-answer" id={answer_id}>
                                        <p>{ item.answer }</p>
                                    </div>
                                }
                            </div>
                        }
                    }).collect::<Html>() }
                </div>
            </div>
        </section>
    }
}
