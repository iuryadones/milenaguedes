use yew::prelude::*;
use crate::models::SiteConfig;

#[function_component(DepoimentosSection)]
pub fn depoimentos_section() -> Html {
    html! {
        <section class="section section-depoimentos">
            <div class="container">
                <h2 class="section-title">{ "O que dizem quem já viveu essa experiência" }</h2>
                <p class="section-subtitle">{ "Em breve, depoimentos reais de clientes que transformaram seu bem-estar" }</p>
                <div class="depoimentos-placeholder">
                    <div class="depoimento-icon">{"💬"}</div>
                    <p class="depoimento-text">
                        { "Quer compartilhar sua experiência? " }
                        <a href={format!("https://wa.me/{}", SiteConfig::PHONE)} target="_blank" rel="noopener noreferrer">
                            { "Envie seu depoimento pelo WhatsApp" }
                        </a>
                        { " e ele pode aparecer aqui!" }
                    </p>
                </div>
            </div>
        </section>
    }
}
