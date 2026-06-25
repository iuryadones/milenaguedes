use yew::prelude::*;
use crate::models::SiteConfig;
use crate::components::whatsapp_button::WhatsAppLink;
use crate::seo::set_page_meta;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    use_effect_with((), |_| {
        set_page_meta(
            "Contato — Milena Guedes | Massoterapia Integrativa",
            "Agende sua sessão pelo WhatsApp, visite o espaço em Candeias — Jaboatão dos Guararapes, ou siga @milenaguedesintegrativa no Instagram.",
        );
        || {}
    });
    html! {
        <>
            <section class="page-header">
                <div class="container">
                    <h1>{ "Contato" }</h1>
                    <p>{ "Entre em contato e comece sua jornada de bem-estar" }</p>
                </div>
            </section>
            <section class="section">
                <div class="container contact-grid">
                    <div class="contact-info">
                        <div class="contact-item">
                            <h3>{ "📱 WhatsApp" }</h3>
                            <p class="contact-phone">{ SiteConfig::PHONE_DISPLAY }</p>
                            <div class="contact-whatsapp-actions">
                                <WhatsAppLink
                                    message={"Olá Milena! Gostaria de agendar uma sessão."}
                                    classes="btn btn-primary"
                                >
                                    { "Quero agendar" }
                                </WhatsAppLink>
                                <WhatsAppLink
                                    message={"Olá Milena! Tenho uma dúvida sobre os serviços."}
                                    classes="btn btn-outline"
                                >
                                    { "Tenho dúvidas" }
                                </WhatsAppLink>
                                <WhatsAppLink
                                    message={"Olá Milena! Gostaria de informações sobre valores e horários."}
                                    classes="btn btn-outline"
                                >
                                    { "Valores e horários" }
                                </WhatsAppLink>
                            </div>
                        </div>
                        <div class="contact-item">
                            <h3>{ "📸 Instagram" }</h3>
                            <p>{ "Acompanhe dicas de bem-estar e autocuidado" }</p>
                            <a href={SiteConfig::INSTAGRAM_URL}
                               target="_blank" rel="noopener noreferrer"
                               class="btn btn-outline">
                                { "@milenaguedesintegrativa" }
                            </a>
                        </div>
                        <div class="contact-item">
                            <h3>{ "📍 Endereço" }</h3>
                            <p>{ SiteConfig::PLACE_NAME }</p>
                            <a href={SiteConfig::PLACE_URL}
                               target="_blank" rel="noopener noreferrer"
                               class="address-link">
                                { SiteConfig::ADDRESS }
                            </a>
                            <a href={SiteConfig::PLACE_URL}
                               target="_blank" rel="noopener noreferrer"
                               class="btn btn-outline btn-sm contato-map-link">
                                { "Abrir no Google Maps" }
                            </a>
                        </div>
                        <div class="contact-item">
                            <h3>{ "🕐 Horários" }</h3>
                            <p>{ SiteConfig::HOURS }</p>
                        </div>
                    </div>
                    <div class="contact-map">
                        <iframe
                            src={SiteConfig::MAP_EMBED}
                            width="100%" height="380"
                            allowfullscreen={true}
                            loading="lazy"
                            referrerpolicy="no-referrer-when-downgrade"
                            title="Mapa do consultório Milena Guedes">
                        </iframe>
                    </div>
                </div>
            </section>
        </>
    }
}
