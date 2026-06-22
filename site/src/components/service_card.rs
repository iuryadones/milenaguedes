use yew::prelude::*;
use crate::models::Service;
use crate::components::whatsapp_button::WhatsAppLink;

#[derive(Properties, PartialEq)]
pub struct ServiceCardProps {
    pub service: Service,
}

#[function_component(ServiceCard)]
pub fn service_card(props: &ServiceCardProps) -> Html {
    let msg = format!(
        "Olá Milena! Tenho interesse no serviço: {}",
        props.service.title
    );

    html! {
        <div class="service-card">
            <div class="service-card-header">
                <div class="service-card-icon">{ props.service.icon }</div>
                <span class="service-card-duration">{ props.service.duration }</span>
            </div>
            <h3 class="service-card-title">{ props.service.title }</h3>
            <p class="service-card-desc">{ props.service.description }</p>
            <WhatsAppLink message={AttrValue::from(msg)} classes="btn btn-outline btn-sm">
                { "Quero agendar" }
            </WhatsAppLink>
        </div>
    }
}
