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
                <div class="instagram-embed">
                    <iframe
                        src="https://www.instagram.com/milenaguedesintegrativa/embed"
                        width="100%"
                        height="480"
                        frameborder="0"
                        scrolling="no"
                        allowtransparency="true"
                        style="border-radius: 16px;"
                        title="Perfil do Instagram @milenaguedesintegrativa"
                    >
                    </iframe>
                </div>
                <div class="text-center" style="margin-top: 1.5rem;">
                    <a
                        href={SiteConfig::INSTAGRAM_URL}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-outline"
                    >
                        { "Seguir @milenaguedesintegrativa" }
                    </a>
                </div>
            </div>
        </section>
    }
}
