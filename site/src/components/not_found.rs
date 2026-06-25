use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::whatsapp_button::WhatsAppLink;
use crate::seo::set_page_meta_with_noindex;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    use_effect_with((), |_| {
        set_page_meta_with_noindex("Página não encontrada — Milena Guedes", "A página que você procura não existe. Volte ao início ou entre em contato pelo WhatsApp.", true);
        || {}
    });
    html! {
        <section class="page-header not-found-section">
            <div class="container text-center">
                <h1 class="not-found-title">{ "404" }</h1>
                <p class="not-found-text">
                    { "Página não encontrada" }
                </p>
                <p class="not-found-text-sm">
                    { "A página que você procura não existe ou foi movida." }
                </p>
                <div class="not-found-actions">
                    <Link<Route> to={Route::Home} classes="btn btn-primary">
                        { "Voltar ao início" }
                    </Link<Route>>
                    <WhatsAppLink
                        message={"Olá Milena! Estou com dúvidas sobre o site."}
                        classes="btn btn-outline"
                    >
                        { "Falar com Milena" }
                    </WhatsAppLink>
                </div>
            </div>
        </section>
    }
}
