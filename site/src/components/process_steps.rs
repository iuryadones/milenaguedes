use yew::prelude::*;
use crate::components::whatsapp_button::WhatsAppLink;

const STEPS: &[(&str, &str, &str)] = &[
    ("📋", "Escolha", "Selecione o serviço que mais combina com o que seu corpo pede."),
    ("💬", "Agende", "Entre em contato pelo WhatsApp e escolha o melhor horário para você."),
    ("🚪", "Chegue", "Venha para um ambiente pensado para o seu acolhimento e relaxamento."),
    ("💆‍♀️", "Relaxe", "Entregue-se ao cuidado. Cada sessão é uma experiência única de renovação."),
];

#[function_component(ProcessSteps)]
pub fn process_steps() -> Html {
    html! {
        <section class="section section-process">
            <div class="container">
                <h2 class="section-title">{ "Como funciona" }</h2>
                <p class="section-subtitle">{ "Simples, rápido e pensado para você" }</p>
                <div class="steps-grid">
                    { STEPS.iter().enumerate().map(|(i, (icon, title, desc))| {
                        html! {
                            <div class="step-card">
                                <div class="step-number">{ i + 1 }</div>
                                <div class="step-icon">{ icon }</div>
                                <h3 class="step-title">{ title }</h3>
                                <p class="step-desc">{ desc }</p>
                            </div>
                        }
                    }).collect::<Html>() }
                </div>
                <div class="text-center process-cta">
                    <WhatsAppLink
                        message={"Olá Milena! Gostaria de agendar minha primeira sessão."}
                        classes="btn btn-primary btn-lg"
                    >
                        { "Agende sua sessão agora" }
                    </WhatsAppLink>
                </div>
            </div>
        </section>
    }
}
