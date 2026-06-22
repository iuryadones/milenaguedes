use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::whatsapp_button::WhatsAppLink;
use crate::router::Route;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="hero">
            <div class="hero-pattern"></div>
            <div class="hero-content">
                <span class="hero-badge">{ "Massoterapia Integrativa" }</span>
                <h1 class="hero-title">
                    { "O alívio que seu corpo merece" }
                </h1>
                <p class="hero-subtitle">
                    { "Dores, estresse e cansaço têm solução. Sessões personalizadas que cuidam de você de dentro para fora." }
                </p>
                <div class="hero-actions">
                    <WhatsAppLink
                        message={"Olá Milena! Gostaria de agendar uma sessão de massoterapia integrativa."}
                        classes="btn btn-primary btn-lg"
                    >
                        { "Agendar sessão" }
                    </WhatsAppLink>
                    <Link<Route> to={Route::Servicos} classes="btn btn-outline btn-lg">
                        { "Ver serviços" }
                    </Link<Route>>
                </div>
            </div>
        </section>
    }
}
