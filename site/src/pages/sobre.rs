use yew::prelude::*;
use crate::components::cta_section::CTASection;
use crate::components::faq_section::FAQSection;
use crate::components::diferenciais::DiferenciaisSection;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <>
            <section class="page-header">
                <div class="container">
                    <h1>{ "Sobre Milena Guedes" }</h1>
                    <p>{ "Cuidado, acolhimento e transformação através da massoterapia integrativa" }</p>
                </div>
            </section>
            <section class="section">
                <div class="container about-content">
                    <div class="about-image">
                        <img src="/assets/images/milena_guedes.jpeg" alt="Milena Guedes — Massoterapeuta Integrativa" class="about-photo" />
                    </div>
                    <div class="about-text">
                        <h2>{ "Massoterapeuta Integrativa" }</h2>
                        <p>
                            { "Milena Guedes é massoterapeuta integrativa especialista em dores emocionais, dores físicas agudas e crônicas, com formação em Medicina Chinesa. Seu trabalho une técnica e sensibilidade para cuidar de você como um todo." }
                        </p>
                        <p>
                            { "A paixão por espiritualidade e autoconhecimento a levou à massoterapia integrativa — um canal de cuidado, acolhimento e cura que vai além do toque. Cada sessão é única, pensada exclusivamente para o que seu corpo precisa naquele momento, com presença, entrega e cuidado genuíno." }
                        </p>
                        <p>
                            { "Seu espaço é mais que um consultório: é um refúgio de relaxamento e renovação, onde técnicas avançadas se encontram com experiências sensoriais para devolver ao corpo a leveza que ele merece." }
                        </p>
                    </div>
                </div>
            </section>
            <section class="section section-philosophy">
                <div class="container">
                    <h2 class="section-title">{ "Minha Filosofia" }</h2>
                    <div class="philosophy-content">
                        <div class="philosophy-item">
                            <span class="philosophy-icon">{ "💚" }</span>
                            <p>{ "Acredito que o corpo guarda tudo o que não foi dito. Cada tensão, cada nó, cada desconforto tem uma história. Minha missão é ajudar você a ouvir o que seu corpo está dizendo." }</p>
                        </div>
                        <div class="philosophy-item">
                            <span class="philosophy-icon">{ "🤝" }</span>
                            <p>{ "Não existem duas pessoas iguais, então não existem duas sessões iguais. Cada atendimento é montado sob medida, respeitando seu tempo, seus limites e suas necessidades." }</p>
                        </div>
                        <div class="philosophy-item">
                            <span class="philosophy-icon">{ "🌱" }</span>
                            <p>{ "O autocuidado não é luxo, é necessidade. Minha função é criar um espaço seguro onde você pode desacelerar, se reconectar e voltar para o mundo mais leve." }</p>
                        </div>
                    </div>
                </div>
            </section>
            <DiferenciaisSection />
            <FAQSection />
            <CTASection
                title="Vamos começar sua jornada de cuidado?"
                subtitle="Agende uma conversa com Milena e descubra o que seu corpo precisa."
                message="Olá Milena! Quero saber mais sobre seu trabalho."
                button="Conversar com Milena"
            />
        </>
    }
}
