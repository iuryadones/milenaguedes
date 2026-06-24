use yew::prelude::*;
use crate::models::SiteConfig;
use crate::components::icons;
use js_sys::encode_uri_component;

#[derive(Properties, PartialEq, Clone)]
pub struct WhatsAppLinkProps {
    pub message: AttrValue,
    pub classes: AttrValue,
    pub children: Children,
}

#[function_component(WhatsAppLink)]
pub fn whatsapp_link(props: &WhatsAppLinkProps) -> Html {
    let encoded = encode_uri_component(&props.message);
    let url = format!("https://wa.me/{}?text={}", SiteConfig::PHONE, &encoded);

    html! {
        <a href={url} target="_blank" rel="noopener noreferrer" class={props.classes.clone()}>
            { for props.children.iter() }
        </a>
    }
}

#[function_component(WhatsAppFloating)]
pub fn whatsapp_floating() -> Html {
    let encoded = encode_uri_component("Olá Milena! Gostaria de saber mais sobre seus serviços.");
    let url = format!("https://wa.me/{}?text={}", SiteConfig::PHONE, &encoded);

    html! {
        <a
            href={url}
            target="_blank"
            rel="noopener noreferrer"
            class="whatsapp-floating"
            aria-label="Fale pelo WhatsApp"
        >
            <svg viewBox="0 0 24 24" width="28" height="28" fill="white">
                <path d={icons::WHATSAPP_SVG}/>
            </svg>
        </a>
    }
}
