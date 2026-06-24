use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::hero::Hero;
use crate::components::cta_section::CTASection;
use crate::components::diferenciais::DiferenciaisSection;
use crate::components::process_steps::ProcessSteps;
use crate::components::instagram_section::InstagramSection;
use crate::components::depoimentos::DepoimentosSection;
use crate::models::SERVICES;
use crate::components::service_card::ServiceCard;
use crate::router::Route;
use crate::seo::set_page_meta;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    use_effect(|| {
        set_page_meta(
            "Milena Guedes — Massoterapia Integrativa",
            "Seu refúgio de relaxamento e renovação. Massoterapia Integrativa e experiências sensoriais em Jaboatão dos Guararapes.",
        );
        || {}
    });

    let featured: Vec<_> = SERVICES.iter().take(3).collect();

    html! {
        <>
            <Hero />
            <DiferenciaisSection />
            <section class="section">
                <div class="container">
                    <h2 class="section-title">{ "Nossos Serviços" }</h2>
                    <p class="section-subtitle">{ "Técnicas integrativas para corpo, mente e espírito" }</p>
                    <div class="services-grid">
                        { featured.into_iter().map(|s| {
                            html! { <ServiceCard key={s.id} service={s.clone()} /> }
                        }).collect::<Html>() }
                    </div>
                    <div class="text-center home-services-link">
                        <Link<Route> to={Route::Servicos} classes="btn btn-outline">
                            { "Ver todos os serviços →" }
                        </Link<Route>>
                    </div>
                </div>
            </section>
            <ProcessSteps />
            <CTASection
                title="Pronto para transformar seu bem-estar?"
                subtitle="Sua primeira sessão está a uma mensagem de distância. Milena espera por você."
                message="Olá Milena! Quero agendar minha primeira sessão."
                button="Agendar agora pelo WhatsApp"
            />
            <InstagramSection />
            <DepoimentosSection />
        </>
    }
}
