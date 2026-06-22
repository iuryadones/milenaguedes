use yew::prelude::*;
use crate::models::SERVICES;
use crate::components::service_card::ServiceCard;
use crate::components::cta_section::CTASection;
use crate::components::faq_section::FAQSection;

#[function_component(ServicesPage)]
pub fn services_page() -> Html {
    html! {
        <>
            <section class="page-header">
                <div class="container">
                    <h1>{ "Serviços" }</h1>
                    <p>{ "Massoterapia Integrativa e experiências sensoriais para o seu bem-estar" }</p>
                </div>
            </section>
            <section class="section">
                <div class="container">
                    <div class="services-grid">
                        { SERVICES.iter().map(|s| {
                            html! { <ServiceCard key={s.id} service={s.clone()} /> }
                        }).collect::<Html>() }
                    </div>
                </div>
            </section>
            <FAQSection />
            <CTASection
                title="Não sabe qual serviço escolher?"
                subtitle="Converse com Milena e descubra a melhor opção para o seu momento."
                message="Olá Milena! Preciso de ajuda para escolher o melhor serviço para mim."
                button="Tirar dúvidas no WhatsApp"
            />
        </>
    }
}
