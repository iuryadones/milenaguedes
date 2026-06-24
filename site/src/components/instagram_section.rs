use yew::prelude::*;
use crate::models::SiteConfig;

#[function_component(InstagramSection)]
pub fn instagram_section() -> Html {
    html! {
        <section class="section section-instagram">
            <div class="container">
                <h2 class="section-title">{ "Acompanhe nas redes" }</h2>
                <p class="section-subtitle">
                    { "Conteúdo diário sobre autocuidado, bem-estar e massoterapia integrativa" }
                </p>
                <div class="instagram-fallback">
                    <span class="instagram-fallback-icon">{ "📸" }</span>
                    <p class="instagram-fallback-text">
                        { "Siga @milenaguedesintegrativa no Instagram e acompanhe dicas diárias de bem-estar, autocuidado e massoterapia integrativa." }
                    </p>
                    <a
                        href={SiteConfig::INSTAGRAM_URL}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-primary"
                    >
                        { "Seguir @milenaguedesintegrativa" }
                    </a>
                </div>
            </div>
        </section>
    }
}
