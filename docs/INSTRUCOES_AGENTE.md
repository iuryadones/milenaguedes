# Instruções para o Agente de Código — Site Milena Guedes

## Stack
- Rust + Yew.rs 0.21 (CSR), yew-router 0.18, Trunk, CSS puro

## Deploy
- **Domínio:** milenaguedes.com
- **master**: código fonte (Rust + CSS + docs + `assets/`)
- **gh-pages**: apenas build estático (`dist/`: index.html, 404.html, .wasm, .js, .css, _redirects, CNAME, sitemap.xml, `assets/`)
- Build: `trunk build --release --public-url /`
- `make deploy` gera `dist/` + `404.html` (cópia do index.html) + `_redirects` + `CNAME`
- DNS: 4 registros A (GitHub IPs) + CNAME www → iuryadones.github.io

### GitHub Pages (configuração única)
1. Repo → Settings → Pages
2. Source: **gh-pages** / **(root)**
3. Custom domain: `milenaguedes.com`
4. HTTPS automático via Let's Encrypt (minutos após DNS propagar)

### DNS (Unstoppable Domains)
- **Antes:** desativar **Domain Forwarding** no painel do Unstoppable (Forwarding → desativar)
- **Depois:** adicionar no DNS Management:

| Tipo | Nome | Valor | TTL |
|------|------|-------|-----|
| `A` | `@` | `185.199.108.153` | 3600 |
| `A` | `@` | `185.199.109.153` | 3600 |
| `A` | `@` | `185.199.110.153` | 3600 |
| `A` | `@` | `185.199.111.153` | 3600 |
| `CNAME` | `www` | `iuryadones.github.io` | 3600 |

---

## Comandos

| Comando | Descrição |
|---------|-----------|
| `make setup` | Instala trunk + target wasm32 |
| `make run` | Servidor dev :8080 |
| `make check` | Verifica compilação (wasm32) |
| `make build` | Build produção → `dist/` |
| `make deploy` | Build + gera `404.html` + `_redirects` + `CNAME` |
| `make clean` | Remove artefatos |

---

## 1. Arquitetura

```
index.html (loading spinner + fonts + favicon 💆‍♀️)
  └── WASM (main.rs → Renderer<App>)
       └── App (BrowserRouter)
            ├── Navbar (fixed, scroll shadow, hamburger, WhatsApp btn)
            ├── <main>
            │    └── Switch<Route>
            │         ├── /        → HomePage      (Hero → Diferenciais → 3 Serviços → ProcessSteps → CTA → Instagram → Depoimentos)
            │         ├── /sobre   → AboutPage     (Header → Bio + Foto → Filosofia → Diferenciais → FAQ → CTA)
│         ├── /servicos → ServicesPage (Header → 11 Cards → FAQ → CTA)
│         ├── /contato → ContactPage   (Header → Info + WhatsApp rápido → Mapa)
│         ├── /404     → NotFoundPage  (404 amigável)
│         └── /*       → NotFoundPage  (catch-all)
├── WhatsAppFloating (FAB canto inferior direito)
├── WhatsAppMobileBar (sticky bar inferior, mobile only)
├── Footer (grid 3-col: brand + serviços + links)
├── LGPDNotice (banner inferior, checa localStorage no mount)
└── MetaTags (set_page_meta via web_sys em cada página)
```

### Hierarquia de Componentes

```
App
├── Navbar (use_route + use_navigator, scroll shadow Closure)
│   ├── Logo (link Home)
│   └── NavLinks + WhatsApp btn
├── Switch<Route>
│   ├── HomePage
│   │   ├── Hero (pattern SVG, badge, 2 CTAs: WhatsApp + Ver Serviços)
│   │   ├── DiferenciaisSection (4 cards: Atendimento, Medicina Chinesa, Ambiente, Sensorial)
│   │   ├── ServiceCard[] (3 destaques)
│   │   ├── ProcessSteps (4 passos: Escolha → Agende → Chegue → Relaxe)
│   │   ├── CTASection (props: title, subtitle, message, button)
│   │   ├── InstagramSection (embed + link)
│   │   └── DepoimentosSection (placeholder)
│   ├── AboutPage
│   │   ├── ImagePlaceholder + Bio
│   │   ├── PhilosophySection (3 cards: filosofia de trabalho)
│   │   ├── DiferenciaisSection
│   │   ├── FAQSection (accordion, 9 perguntas)
│   │   └── CTASection
│   ├── ServicesPage
│   │   ├── ServiceCard[] (11 serviços completos, duration badge)
│   │   ├── FAQSection (accordion, 9 perguntas)
│   │   └── CTASection
│   ├── ContactPage
│   │   ├── WhatsApp (3 atalhos: Agendar, Dúvidas, Valores)
│   │   ├── Instagram
│   │   ├── Endereço + Horários
│   │   └── Google Maps (embed, query-based, sem API key)
│   └── NotFoundPage (404 + links Home/WhatsApp)
├── Footer (3-col grid: Brand+WhatsApp | Serviços(5) | Links)
├── WhatsAppFloating (FAB position:fixed, wa.me)
├── WhatsAppMobileBar (display:none desktop, flex mobile sticky bottom)
└── LGPDNotice (banner fixed bottom, localStorage)
```

---

## 2. Props API

| Componente | Prop | Tipo | Default | Descrição |
|-----------|------|------|---------|-----------|
| `WhatsAppLink` | `message` | `AttrValue` | — | Texto pré-preenchido do WhatsApp |
| `WhatsAppLink` | `classes` | `AttrValue` | — | Classes CSS |
| `WhatsAppLink` | `children` | `Children` | — | Conteúdo do link |
| `WhatsAppFloating` | — | — | — | FAB fixo (usa SiteConfig) |
| `ServiceCard` | `service` | `Service` | — | id, title, description, icon, duration |
| `Hero` | — | — | — | Pattern SVG + 2 CTAs |
| `CTASection` | `title` | `AttrValue` | "Sua jornada..." | Título |
| `CTASection` | `subtitle` | `AttrValue` | "Entre em contato..." | Subtítulo |
| `CTASection` | `message` | `AttrValue` | "Olá Milena..." | Mensagem WhatsApp |
| `CTASection` | `button` | `AttrValue` | "Fale com Milena..." | Texto do botão |
| `DiferenciaisSection` | — | — | — | Usa `DIFERENCIAIS` (models.rs) |
| `ProcessSteps` | — | — | — | Steps fixos + CTA |
| `InstagramSection` | — | — | — | Embed + link |
| `DepoimentosSection` | — | — | — | Placeholder |
| `FAQSection` | — | — | — | Accordion, usa `FAQ` (models.rs) |
| `Navbar` | — | — | — | use_route + use_navigator |
| `Footer` | — | — | — | Grid 3-col, linka Serviços |
| `NotFoundPage` | — | — | — | 404 amigável |
| `LGPDNotice` | — | — | — | Banner fixo + localStorage |

---

## 3. Modelos de Dados — `src/models.rs`

```rust
pub struct Service {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,  // benefit-driven copy
    pub icon: &'static str,
    pub duration: &'static str,     // ex: "60 min"
}

pub const SERVICES: &[Service];  // 11 serviços

pub struct FAQItem {
    pub question: &'static str,
    pub answer: &'static str,
}
pub const FAQ: &[FAQItem];  // 9 perguntas

pub struct Diferencial {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}
pub const DIFERENCIAIS: &[Diferencial];  // 4 diferenciais

pub struct SiteConfig;  // constants: PHONE, INSTAGRAM_URL, ADDRESS, HOURS, MAP_EMBED
```

### Serviços (descrições persuasivas):

| ID | Título | Duração |
|----|--------|---------|
| terapeutica | Massagem Terapêutica | 60 min |
| relaxante | Massagem Relaxante | 60 min |
| miofascial | Liberação Miofascial | 60 min |
| drenagem | Drenagem Linfática | 60 min |
| ventosa | Ventosaterapia | 60 min |
| ritual-premium | Ritual Relax Premium | 90 min |
| personalizadas | Terapias Personalizadas | 60-90 min |
| medicina-chinesa | Medicina Chinesa | 60 min |
| sensorial | Experiências Sensoriais | 60 min |
| tantrica | Terapia Tântrica | 90 min |
| escuta-ativa | Escuta Ativa | 60 min |

---

## 4. Estrutura de Diretórios

```
site/
├── Cargo.toml / Trunk.toml / index.html / Makefile
├── src/
│   ├── main.rs / app.rs / router.rs / models.rs / seo.rs
│   ├── components/
│   │   ├── mod.rs (registro público)
│   │   ├── navbar.rs, hero.rs, service_card.rs
│   │   ├── whatsapp_button.rs (WhatsAppLink + WhatsAppFloating)
│   │   ├── cta_section.rs (props para variantes)
│   │   ├── footer.rs (grid 3-col com serviços, ano dinâmico)
│   │   ├── diferenciais.rs (4 cards)
│   │   ├── process_steps.rs (4 passos + CTA)
│   │   ├── instagram_section.rs (embed + link)
│   │   ├── depoimentos.rs (placeholder)
│   │   ├── faq_section.rs (accordion)
│   │   ├── not_found.rs (404)
│   │   └── lgpd_notice.rs (banner LGPD, checa localStorage)
│   └── pages/
│       ├── home.rs (Hero → Diferenciais → 3 Serviços → Steps → CTA → Instagram → Depoimentos)
│       ├── sobre.rs (Bio → Filosofia → Diferenciais → FAQ → CTA)
│       ├── servicos.rs (11 Cards → FAQ → CTA)
│       └── contato.rs (Info + 3 atalhos WhatsApp → Mapa)
├── css/styles.css (~1068 linhas)
├── assets/images/ (milena_guedes.jpeg real, 600×600)
├── sitemap.xml (4 URLs)
└── 404.html (gerado no deploy, fallback SPA)
```

---

## 5. Decisões Técnicas

### Navegação (Navbar)
- `<Link>` do yew-router 0.18 não suporta `onclick`. Usar `use_navigator()` + `<a>` com `Callback::from(move |e: MouseEvent| { e.prevent_default(); navigator.push(&target); })`.
- Active link via `use_route::<Route>()` comparando com o enum.
- Scroll shadow via `Closure<dyn Fn()>` + `window.set_onscroll`.

### SPA Routing (GitHub Pages)
- `_redirects` (`/* /index.html 200`) funciona em alguns casos mas GH Pages nem sempre o processa.
- `404.html` (cópia do `index.html` com `<base href="/">`) é o fallback confiável.
- Gerado automaticamente por `make deploy`.
- O `<base href="/">` garante que assets (JS, WASM, CSS) carreguem da raiz independente da rota.

### SEO por Página (`set_page_meta`)
- Módulo `src/seo.rs` com função `set_page_meta(title, description)`.
- Chamada via `use_effect` em cada página (Home, Sobre, Serviços, Contato, 404).
- Atualiza `<title>`, `<meta name="description">`, `<meta property="og:title">` e `<meta property="og:description">` via `web_sys::Document`.
- Cada página tem title + description únicos.

### LGPD Notice
- `web_sys::window().local_storage()` retorna `Result<Option<Storage>, JsValue>`.
- Padrão: `if let Some(Ok(Some(storage))) = window().map(|w| w.local_storage())`.
- No mount, verifica `localStorage.getItem("lgpd-accepted")` — se existir, banner não aparece.
- `use_state` é inicializado com o resultado da checagem, não com `true` fixo.

### WhatsApp
- Link usa `js_sys::encode_uri_component` no texto da mensagem.
- URL: `https://wa.me/{PHONE}?text={encoded}`.
- FAB global + mobile sticky bar + inline buttons.

### Google Maps
- Embed sem API key: `https://www.google.com/maps?q={endereço_url}&output=embed`.

### Footer
- Ano do copyright dinâmico via `js_sys::Date::new_0().get_full_year()` — não precisa atualizar manualmente.

### CSS
- Scroll-triggered animations: usar `IntersectionObserver` via web-sys ou fadeUp universal.
- Alternating section backgrounds: `.section:nth-child(even)`.
- Mobile sticky WhatsApp bar: `display:none` desktop, `display:flex` mobile.
- Hero pattern SVG inline via `background-image: url("data:image/svg+xml,...")`.
- Service cards com gradient top border (`::before`).
- FAQ accordion com estado local (`use_state<Option<usize>>`).

### Atalho `prop_or` com AttrValue
- `AttrValue` em yew 0.21 é `IString`. Usar `#[prop_or_else(|| AttrValue::from("texto"))]` em vez de `#[prop_or("texto")]`.
- Para children dinâmicos: passar valor clonado (`{ props.button.clone() }`) em vez de referência.

---

## 6. Customização

### Adicionar/editar serviços
```rust
// src/models.rs — array SERVICES
Service {
    id: "meu-servico",
    title: "Meu Serviço",
    description: "Descrição persuasiva focada em benefício.",
    icon: "🌟",
    duration: "60 min",
},
```

### Adicionar FAQ
```rust
// src/models.rs — array FAQ
FAQItem {
    question: "Minha pergunta?",
    answer: "Minha resposta tranquilizadora.",
},
```

### Adicionar Diferencial
```rust
// src/models.rs — array DIFERENCIAIS
Diferencial {
    icon: "🌟",
    title: "Meu Diferencial",
    description: "Descrição do diferencial.",
},
```

### Alterar cores
```css
/* css/styles.css — :root */
--gold: #C4A33D;         /* botões, CTAs, acentos */
--gold-dark: #9E8328;    /* hover/active */
--gold-light: #D4B351;   /* badges, tags sutis */
--beige: #F0EADE;        /* fundos alternados */
--beige-light: #F8F4ED;  /* fundo principal */
--black: #1C1C1C;        /* texto, footer */
--black-soft: #3A3A3A;   /* texto secundário (contraste WCAG AA) */
--white: #FFFFFF;         /* cards */
--whatsapp-green: #25D366; /* mantido */
```

### Trocar foto placeholder
1. Adicionar foto em `assets/images/` (ex: `nome.jpg`)
2. Em `index.html`, adicionar: `<link data-trunk rel="copy-dir" href="assets" />`
3. Em `sobre.rs`: `<img src="/assets/images/nome.jpg" alt="..." class="about-photo" />`

### Adicionar depoimentos reais
1. Criar array `DEPOIMENTOS` em `models.rs` com `autor`, `texto`, `foto`.
2. Substituir `DepoimentosSection` placeholder por carousel real.

---

## 7. Troubleshooting

| Erro | Causa | Solução |
|------|-------|---------|
| `expected IString, found &str` | `#[prop_or("texto")]` com `AttrValue` | Usar `#[prop_or_else(|| AttrValue::from("texto"))]` |
| `cannot find type JsCast` | Falta import | `use wasm_bindgen::JsCast;` |
| `Closure cannot be captured` | Escopo do closure | Clonar variáveis com `.clone()` antes do `move` |
| Rota SPA quebra no GH Pages | `_redirects` não aplicado ou cache do GH Pages | `make deploy` gera `_redirects` + `404.html` (fallback). `404.html` é cópia do `index.html` com `<base href="/">` |
| Link não navega | Usou `<Link>` sem `onclick` | Usar `<a>` + `use_navigator()` + `prevent_default` |
| `Children` vs `Html` | Props de children | Usar `Children` no struct e `for props.children.iter()` no template |

---

## 8. Fluxo de Desenvolvimento

```bash
make setup      # Instala trunk + target (primeira vez)
make run        # http://localhost:8080
# ... editar código ...
make check      # Verifica compilação
make build      # Build produção
make deploy     # Build + _redirects → copiar dist/ para gh-pages
```
