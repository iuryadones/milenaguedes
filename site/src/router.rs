use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sobre")]
    Sobre,
    #[at("/servicos")]
    Servicos,
    #[at("/contato")]
    Contato,
    #[at("/404")]
    NotFound,
    #[at("/{*:rest}")]
    NotRoute,
}
