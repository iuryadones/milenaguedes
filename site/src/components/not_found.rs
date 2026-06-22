use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::whatsapp_button::WhatsAppLink;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <section class="page-header" style="min-height: 60vh; display: flex; align-items: center; justify-content: center;">
            <div class="container" style="text-align: center;">
                <h1 style="font-size: 4rem; margin-bottom: 0.5rem;">{ "404" }</h1>
                <p style="font-size: 1.2rem; color: var(--black-soft, #5A5A5A); margin-bottom: 1rem;">
                    { "Página não encontrada" }
                </p>
                <p style="margin-bottom: 2rem; color: var(--black-soft, #5A5A5A);">
                    { "A página que você procura não existe ou foi movida." }
                </p>
                <div style="display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;">
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
