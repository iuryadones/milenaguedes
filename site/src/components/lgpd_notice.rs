use yew::prelude::*;

#[function_component(LGPDNotice)]
pub fn lgpd_notice() -> Html {
    let visible = use_state(|| true);

    if !*visible {
        return html! {};
    }

    let dismiss = {
        let visible = visible.clone();
        Callback::from(move |_| {
            visible.set(false);
            if let Some(Ok(Some(storage))) = web_sys::window()
                .map(|w| w.local_storage())
            {
                let _ = storage.set_item("lgpd-accepted", "true");
            }
        })
    };

    html! {
        <div class="lgpd-notice">
            <p>
                { "Este site utiliza cookies para melhorar sua experiência. Ao continuar navegando, você concorda com nossa " }
                <a href="https://www.gov.br/esporte/pt-br/acesso-a-informacao/lgpd" target="_blank" rel="noopener noreferrer">
                    { "Política de Privacidade" }
                </a>
                { "." }
            </p>
            <button class="btn btn-sm btn-primary" onclick={dismiss}>
                { "Aceitar" }
            </button>
        </div>
    }
}
