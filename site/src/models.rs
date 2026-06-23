#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub duration: &'static str,
}

pub const SERVICES: &[Service] = &[
    Service {
        id: "terapeutica",
        title: "Massagem Terapêutica",
        description: "Suas dores musculares e tensões crônicas têm solução. Técnicas precisas que devolvem a liberdade de movimento que você perdeu.",
        icon: "💆",
        duration: "60 min",
    },
    Service {
        id: "relaxante",
        title: "Massagem Relaxante",
        description: "Desligue o mundo lá fora e reconecte-se com você. O toque suave que acalma a mente e renova suas energias.",
        icon: "🫧",
        duration: "60 min",
    },
    Service {
        id: "miofascial",
        title: "Liberação Miofascial",
        description: "Liberte seu corpo das amarras da tensão. Restaure sua mobilidade e sinta cada músculo respirar de novo.",
        icon: "🔄",
        duration: "60 min",
    },
    Service {
        id: "drenagem",
        title: "Drenagem Linfática",
        description: "Diga adeus ao inchaço e à sensação de peso. Seu corpo mais leve, renovado e cheio de vitalidade.",
        icon: "🌊",
        duration: "60 min",
    },
    Service {
        id: "ventosa",
        title: "Ventosaterapia",
        description: "Ative a circulação e liberte dores profundas. Uma técnica ancestral que seu corpo agradece.",
        icon: "🔥",
        duration: "60 min",
    },
    Service {
        id: "ritual-premium",
        title: "Ritual Relax Premium",
        description: "Presenteie-se com o melhor do cuidado. Uma experiência sensorial exclusiva que transforma o seu dia.",
        icon: "✨",
        duration: "90 min",
    },
    Service {
        id: "personalizadas",
        title: "Terapias Personalizadas",
        description: "Cada corpo é único. Sua sessão montada sob medida para exatamente o que você precisa agora.",
        icon: "🎯",
        duration: "60-90 min",
    },
    Service {
        id: "medicina-chinesa",
        title: "Medicina Chinesa",
        description: "Equilíbrio que vem da ancestralidade. Trate a causa raiz e harmonize corpo, mente e espírito.",
        icon: "☯️",
        duration: "60 min",
    },
    Service {
        id: "sensorial",
        title: "Experiências Sensoriais",
        description: "Uma viagem pelos sentidos. Aromas, sons e toques que renovam cada célula do seu ser.",
        icon: "🌸",
        duration: "60 min",
    },
    Service {
        id: "tantrica",
        title: "Terapia Tântrica",
        description: "Um convite para desacelerar, sentir e se reconectar com o próprio corpo. Liberação de bloqueios emocionais, redução da ansiedade e aumento da autoestima em um espaço seguro e acolhedor.",
        icon: "🕯️",
        duration: "90 min",
    },
    Service {
        id: "escuta-ativa",
        title: "Escuta Ativa",
        description: "Suas emoções falam, mas seu corpo sente primeiro. Em um espaço seguro, livre de julgamentos, Milena ouve o que sua história tem a dizer e, com técnicas de psicanálise, ajuda você a liberar tensões emocionais que travam seu corpo. Uma conversa que cura.",
        icon: "🎧",
        duration: "60 min",
    },
];

pub struct FAQItem {
    pub question: &'static str,
    pub answer: &'static str,
}

pub const FAQ: &[FAQItem] = &[
    FAQItem {
        question: "Preciso tirar a roupa para receber a massagem?",
        answer: "Não necessariamente. Você fica confortavelmente coberto com lençóis e apenas a área sendo trabalhada é descoberta. O importante é que você se sinta seguro e confortável durante toda a sessão.",
    },
    FAQItem {
        question: "Quanto tempo dura cada sessão?",
        answer: "As sessões duram entre 60 e 90 minutos, dependendo da técnica escolhida e da sua necessidade no momento. Na sessão personalizada, o tempo é ajustado para você.",
    },
    FAQItem {
        question: "Posso ir com dor ou lesão?",
        answer: "Sim! A massoterapia terapêutica é indicada exatamente para dores e tensões. Ao agendar, avise sobre suas condições para que Milena prepare a sessão ideal para o seu caso.",
    },
    FAQItem {
        question: "O que preciso levar?",
        answer: "Apenas você. O espaço oferece tudo: maca, óleos essenciais, toalhas e um ambiente acolhedor. Se possível, venha com roupa confortável.",
    },
    FAQItem {
        question: "Tem contraindicações?",
        answer: "Algumas condições como febre, infecções agudas, trombose ativa ou cirurgias recentes requerem avaliação prévia. Ao agendar, conte seu histórico de saúde para total segurança.",
    },
    FAQItem {
        question: "Nunca fiz massagem antes. Tem problema?",
        answer: "Nenhum! Muitos clientes chegam sem nunca ter feito e se encantam. Milena guia tudo com calma, explicando cada passo. Sua primeira vez será uma experiência inesquecível.",
    },
    FAQItem {
        question: "O que é a Terapia Tântrica?",
        answer: "A Terapia Tântrica é um convite para desacelerar, sentir e se reconectar com o próprio corpo. Diferente do que muitos imaginam, é uma prática de autoconhecimento e liberação emocional que ajuda a reduzir ansiedade, liberar bloqueios e aumentar a autoestima — tudo em um ambiente seguro, respeitoso e acolhedor.",
    },
    FAQItem {
        question: "Preciso tirar a roupa na Terapia Tântrica?",
        answer: "Não. Você permanece coberto com lençóis durante toda a sessão, e apenas a área sendo trabalhada é descoberta. O respeito ao seu limite e conforto é absoluto. Milena conduz tudo com total transparência e cuidado, garantindo que você se sinta segura em cada etapa.",
    },
    FAQItem {
        question: "O que é a Escuta Ativa e como funciona?",
        answer: "A Escuta Ativa é um espaço seguro onde Milena ouve você sem julgamentos, com acolhimento e presença. Combinando técnicas de psicanálise com a escuta terapêutica, a sessão ajuda a liberar tensões emocionais que seu corpo guarda — muitas vezes sem que você perceba. Uma conversa que cura.",
    },
];

pub struct Diferencial {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

pub const DIFERENCIAIS: &[Diferencial] = &[
    Diferencial {
        icon: "🎯",
        title: "Atendimento Único",
        description: "Cada sessão é montada exclusivamente para o que seu corpo precisa naquele momento. Sem protocolos fixos, sem receitas prontas — você é única.",
    },
    Diferencial {
        icon: "☯️",
        title: "Medicina Chinesa Integrada",
        description: "Formação em Medicina Chinesa que permite tratar a causa raiz dos desconfortos, não apenas os sintomas. Equilíbrio energético de verdade.",
    },
    Diferencial {
        icon: "🌿",
        title: "Ambiente Acolhedor",
        description: "Um espaço pensado para você desacelerar. Aromas, iluminação e sons criam o cenário perfeito para seu cuidado e relaxamento.",
    },
    Diferencial {
        icon: "✨",
        title: "Experiência Sensorial",
        description: "Mais que uma massagem: uma jornada que combina toque, aromaterapia e presença para renovar corpo e mente por completo.",
    },
];

pub struct SiteConfig;

impl SiteConfig {
    pub const PHONE: &'static str = "5581996589013";
    pub const PHONE_DISPLAY: &'static str = "(81) 9.9658-9013";
    pub const INSTAGRAM_URL: &'static str = "https://www.instagram.com/milenaguedesintegrativa/";
    pub const ADDRESS: &'static str = "Rua Major Médico Vicente da Fonseca de Matos, n. 335 - AP 02, Candeias, Jaboatão dos Guararapes - PE, 54440-370";
    pub const PLACE_NAME: &'static str = "Espaço Milena Guedes Massoterapia";
    pub const PLACE_URL: &'static str = "https://www.google.com/maps/place/?q=place_id:0x7aae1b51cd72ec3:0x93452840940145b8";
    pub const HOURS: &'static str = "Segunda a Sexta: 12h – 20h";
    pub const MAP_EMBED: &'static str = "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3949.77!2d-34.9255419!3d-8.2061566!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x7aae1b51cd72ec3%3A0x93452840940145b8!2sEspa%C3%A7o+Milena+Guedes+Massoterapia!5e0!3m2!1spt-BR!2sbr";
}
