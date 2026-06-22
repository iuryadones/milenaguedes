use yew::prelude::*;
use crate::models::DIFERENCIAIS;

#[function_component(DiferenciaisSection)]
pub fn diferenciais_section() -> Html {
    html! {
        <section class="section section-diferenciais">
            <div class="container">
                <h2 class="section-title">{ "Por que escolher Milena Guedes?" }</h2>
                <p class="section-subtitle">{ "O cuidado que faz diferença no seu bem-estar" }</p>
                <div class="diferenciais-grid">
                    { DIFERENCIAIS.iter().map(|d| {
                        html! {
                            <div class="diferencial-card">
                                <div class="diferencial-icon">{ d.icon }</div>
                                <h3 class="diferencial-title">{ d.title }</h3>
                                <p class="diferencial-desc">{ d.description }</p>
                            </div>
                        }
                    }).collect::<Html>() }
                </div>
            </div>
        </section>
    }
}
