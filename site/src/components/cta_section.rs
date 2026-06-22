use yew::prelude::*;
use crate::components::whatsapp_button::WhatsAppLink;

#[derive(Properties, PartialEq)]
pub struct CTASectionProps {
    #[prop_or_else(|| AttrValue::from("Sua jornada de bem-estar começa aqui"))]
    pub title: AttrValue,
    #[prop_or_else(|| AttrValue::from("Entre em contato e descubra uma experiência única de cuidado e renovação."))]
    pub subtitle: AttrValue,
    #[prop_or_else(|| AttrValue::from("Olá Milena! Quero agendar minha primeira sessão."))]
    pub message: AttrValue,
    #[prop_or_else(|| AttrValue::from("Fale com Milena pelo WhatsApp"))]
    pub button: AttrValue,
}

#[function_component(CTASection)]
pub fn cta_section(props: &CTASectionProps) -> Html {
    html! {
        <section class="cta-section">
            <h2>{ &props.title }</h2>
            <p>{ &props.subtitle }</p>
            <WhatsAppLink
                message={props.message.clone()}
                classes="btn btn-primary btn-lg"
            >
                { props.button.clone() }
            </WhatsAppLink>
        </section>
    }
}
