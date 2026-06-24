use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    navbar::Navbar,
    footer::Footer,
    whatsapp_button::WhatsAppFloating,
    not_found::NotFoundPage,
    lgpd_notice::LGPDNotice,
    icons,
};
use crate::pages::{home::HomePage, sobre::AboutPage, servicos::ServicesPage, contato::ContactPage};
use crate::router::Route;
use crate::models::SiteConfig;

fn scroll_to_top() {
    let window = web_sys::window().expect("no window");
    window.scroll_to_with_x_and_y(0.0, 0.0);
}

fn switch(route: Route) -> Html {
    scroll_to_top();
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Sobre => html! { <AboutPage /> },
        Route::Servicos => html! { <ServicesPage /> },
        Route::Contato => html! { <ContactPage /> },
        Route::NotFound | Route::NotRoute => html! { <NotFoundPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <a href="#main-content" class="skip-link">
                { "Pular para o conteúdo" }
            </a>
            <Navbar />
            <main id="main-content">
                <Switch<Route> render={switch} />
            </main>
            <Footer />
            <WhatsAppFloating />
            <a
                href={format!("https://wa.me/{}?text={}", SiteConfig::PHONE, "Olá Milena! Gostaria de agendar uma sessão.")}
                target="_blank"
                rel="noopener noreferrer"
                class="whatsapp-mobile-bar"
            >
                <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
                    <path d={icons::WHATSAPP_SVG}/>
                </svg>
                { " Fale com Milena pelo WhatsApp" }
            </a>
            <LGPDNotice />
        </BrowserRouter>
    }
}
