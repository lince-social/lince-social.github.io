use std::collections::HashMap;

/// Translation structure for easy AI agent updates
/// To update translations:
/// 1. Modify the English (en) text as the source of truth
/// 2. Ask an AI agent to update pt_br and zh translations to match
/// 3. Each field is clearly labeled for context
#[derive(Clone)]
pub struct Translations {
    pub lang_code: &'static str,
    pub lang_name: &'static str,

    // Navigation
    pub nav_home: &'static str,
    pub nav_about: &'static str,
    pub nav_github: &'static str,
    pub nav_download: &'static str,
    pub nav_theme: &'static str,

    // Hero Section
    pub hero_tagline: &'static str,
    pub hero_title: &'static str,
    pub hero_subtitle: &'static str,
    pub hero_download: &'static str,
    pub hero_docs: &'static str,
    pub hero_learn_more: &'static str,

    // Index Page - Open Source Section
    pub opensource_title: &'static str,
    pub opensource_desc: &'static str,
    pub opensource_free_title: &'static str,
    pub opensource_free_desc: &'static str,
    pub opensource_community_title: &'static str,
    pub opensource_community_desc: &'static str,
    pub opensource_privacy_title: &'static str,
    pub opensource_privacy_desc: &'static str,

    // Index Page - Goal Summary
    pub goal_title: &'static str,
    pub goal_desc: &'static str,
    pub goal_registry_title: &'static str,
    pub goal_registry_desc: &'static str,
    pub goal_needs_title: &'static str,
    pub goal_needs_desc: &'static str,
    pub goal_automation_title: &'static str,
    pub goal_automation_desc: &'static str,

    // Index Page - Links
    pub links_title: &'static str,
    pub link_source: &'static str,
    pub link_download: &'static str,
    pub link_readme: &'static str,
    pub link_issues: &'static str,
    pub link_discussions: &'static str,
    pub link_contributing: &'static str,
    pub link_website: &'static str,
    pub link_org: &'static str,

    // About Page
    pub about_title: &'static str,
    pub about_intro: &'static str,

    pub about_mission_title: &'static str,
    pub about_mission_text: &'static str,

    pub about_registry_title: &'static str,
    pub about_registry_text: &'static str,

    pub about_needs_title: &'static str,
    pub about_needs_text: &'static str,

    pub about_automation_title: &'static str,
    pub about_automation_text: &'static str,

    pub about_dna_title: &'static str,
    pub about_dna_text: &'static str,

    pub about_economic_title: &'static str,
    pub about_economic_text: &'static str,

    pub about_tech_title: &'static str,
    pub about_tech_text: &'static str,

    pub about_philosophy_title: &'static str,
    pub about_philosophy_text: &'static str,

    pub about_get_started: &'static str,
    pub about_download: &'static str,
    pub about_read_docs: &'static str,
    pub about_quick_start: &'static str,
    pub about_join_community: &'static str,

    // Footer
    pub footer_resources: &'static str,
    pub footer_documentation: &'static str,
    pub footer_downloads: &'static str,
    pub footer_issues: &'static str,
    pub footer_community: &'static str,
    pub footer_org: &'static str,
    pub footer_contributing: &'static str,
    pub footer_legal: &'static str,
    pub footer_license: &'static str,
    pub footer_copyright: &'static str,
}

pub fn get_translations() -> HashMap<&'static str, Translations> {
    let mut map = HashMap::new();

    // ============================================================
    // ENGLISH (Source of Truth)
    // ============================================================
    map.insert("en", Translations {
        lang_code: "en",
        lang_name: "English",

        // Navigation
        nav_home: "Home",
        nav_about: "About",
        nav_github: "GitHub",
        nav_download: "Download",
        nav_theme: "Theme",

        // Hero Section
        hero_tagline: "Open Source • Non-Profit • Local First • Your Data, Your Rules",
        hero_title: "Lince",
        hero_subtitle: "A powerful open-source tool for registry, interconnection, and automation of Needs and Contributions. Built by the community, for the community — forever free, forever yours.",
        hero_download: "Download Lince",
        hero_docs: "Documentation",
        hero_learn_more: "Learn More",

        // Index Page - Open Source Section
        opensource_title: "Open Source, Non-Profit, Forever Free",
        opensource_desc: "Lince is built on principles of transparency, community ownership, and digital freedom. No venture capital, no hidden agendas — just software that serves you.",
        opensource_free_title: "100% Free & Open Source",
        opensource_free_desc: "Every line of code is open for inspection, modification, and redistribution. No premium tiers, no paywalls, no artificial limitations. The complete source code is available under a permissive license.",
        opensource_community_title: "Community Driven",
        opensource_community_desc: "Development is guided by user needs, not profit margins. Features are prioritized based on community feedback and real-world use cases. Everyone can contribute code, documentation, or ideas.",
        opensource_privacy_title: "Privacy by Design",
        opensource_privacy_desc: "Your data stays on your device. No cloud accounts, no telemetry, no tracking. Lince works entirely offline. You own your data — export it, back it up, delete it. Complete control.",

        // Index Page - Goal Summary
        goal_title: "What is Lince?",
        goal_desc: "At its core, Lince is a personal database for managing anything through the lens of Needs and Contributions.",
        goal_registry_title: "Personal Registry",
        goal_registry_desc: "Create a local SQLite database (your 'DNA') that stores everything: tasks, habits, inventories, projects. Your personal command center, stored as a single portable file.",
        goal_needs_title: "Needs & Contributions",
        goal_needs_desc: "A simple but powerful concept: negative quantities are Needs (-2 Apples means you need 2), positive are Contributions. This model powers tracking, automation, and trades.",
        goal_automation_title: "Smart Automation",
        goal_automation_desc: "Set recurring needs, trigger system commands based on states, automate daily routines. From habit tracking to computer automation — if it's predictable, Lince can handle it.",

        // Index Page - Links
        links_title: "Quick Links",
        link_source: "Source Code",
        link_download: "Download Latest",
        link_readme: "README",
        link_issues: "Report Issues",
        link_discussions: "Discussions",
        link_contributing: "Contributing Guide",
        link_website: "Website Source",
        link_org: "GitHub Organization",

        // About Page
        about_title: "About Lince",
        about_intro: "Lince is more than software — it's a philosophy of personal data ownership, transparent development, and community-driven innovation. Here's everything you need to know.",

        about_mission_title: "Our Mission",
        about_mission_text: "We believe personal productivity tools should be owned by users, not corporations. Lince exists to provide a powerful, flexible, and completely free alternative to proprietary task managers, habit trackers, and automation tools. We're committed to: never charging for core features, never selling user data (we don't collect any), keeping development open and transparent, empowering users with full control over their digital lives. This isn't a business — it's a community project. We're funded by donations and volunteer contributions, not investors seeking returns.",

        about_registry_title: "The DNA Registry",
        about_registry_text: "When you first launch Lince, it creates a personal database stored locally on your system. On UNIX systems (Linux, macOS), it's located at ~/.config/lince/lince.db. This SQLite database is your 'DNA' — a flexible data structure that can be shaped to represent virtually anything. Tasks, habits, inventories, projects, economic transactions, automation rules — your DNA adapts to your use case. The beauty of SQLite is portability: your entire personal system is a single file you can backup, sync, copy, or share. No cloud lock-in, no account required. Your data travels with you.",

        about_needs_title: "The Needs & Contributions Model",
        about_needs_text: "Every behavior in Lince reduces to a simple concept: Quantity. Negative values represent Needs — things you must acquire or accomplish. Positive values represent Contributions — things you have or have completed. Need 2 apples? That's -2 Apples in your DNA. Ate one? Increment by 1 to -1 Apples. This elegantly simple model unifies: Task management (Need = -1 'Complete report'), Habit tracking (Daily reset to -1 'Exercise'), Inventory management (Need = -3 'Milk gallons'), Resource tracking (Contribution = +500 'Budget dollars'). Everything is a number. Simple, but incredibly powerful.",

        about_automation_title: "Automation & Triggers",
        about_automation_text: "If there's predictability in your life, Lince can automate it. Fixed Frequencies automatically adjust quantities on a schedule — reset your 'Study Spanish' habit to -1 every morning, decrement 'Days until deadline' every 24 hours. State Triggers execute system commands when conditions are met — when 'Focus Mode' equals -1, run a script to block distracting websites; when 'Backup Needed' reaches -7, trigger your backup routine. The automation layer connects your personal data to your computer's capabilities. Combined with shell scripts, this becomes incredibly powerful: notifications, application control, system maintenance, custom workflows. The limit is your imagination.",

        about_dna_title: "Shareable DNA Patterns",
        about_dna_text: "Great organizational systems often die with their creators. Knowledge gets lost, workflows forgotten. Lince solves this with shareable DNA patterns — pre-built database templates for common activities. Import a morning routine DNA to kickstart your day. Load a project management template with task dependencies. Get a home inventory system with common items pre-configured. Share your custom productivity DNA with the community. Patterns let you start from proven systems rather than building from scratch. As the community grows, so does the library of shared patterns.",

        about_economic_title: "Economic Transactions",
        about_economic_text: "The Needs/Contributions model naturally extends to economics. Trade is simply the transfer of quantities between parties. Model purchases, sales, debts, and credits. Track inventory across multiple locations. Manage shared resources in households or organizations. Handle multi-party transactions with clear accountability. Lince isn't accounting software, but it can model economic relationships when you need it.",

        about_tech_title: "Technical Foundation",
        about_tech_text: "Lince is built with Rust, chosen for performance, reliability, and memory safety. The compile-time guarantees prevent entire classes of bugs common in other languages. Data lives in SQLite, the most deployed database in the world. Battle-tested, lightweight, and completely self-contained — no database server required. The architecture is intentionally simple: a local binary, a local database, local processing. No microservices, no containers, no cloud dependencies. Install and run. Works offline forever. The codebase is thoroughly documented and welcoming to contributors. Whether you're fixing bugs, adding features, or learning Rust, there's a place for you.",

        about_philosophy_title: "Our Philosophy",
        about_philosophy_text: "Software should serve users, not exploit them. Productivity tools shouldn't require subscriptions, shouldn't harvest data, shouldn't lock you into ecosystems. We build Lince because we want to use it ourselves. Every feature exists because a real person needed it. Every design decision prioritizes user freedom. This is software by people, for people. Join us in building something different. Something that respects you. Something that's truly yours.",

        about_get_started: "Get Started with Lince",
        about_download: "Download Latest Release",
        about_read_docs: "Read the Documentation",
        about_quick_start: "Quick Start Guide",
        about_join_community: "Join the Community",

        // Footer
        footer_resources: "Resources",
        footer_documentation: "Documentation",
        footer_downloads: "Downloads",
        footer_issues: "Issue Tracker",
        footer_community: "Community",
        footer_org: "GitHub Organization",
        footer_contributing: "Contributing",
        footer_legal: "Legal",
        footer_license: "License",
        footer_copyright: "Copyleft 2023-2026 Lince. Built with love and Rust.",
    });

    // ============================================================
    // BRAZILIAN PORTUGUESE
    // ============================================================
    map.insert("pt-br", Translations {
        lang_code: "pt-br",
        lang_name: "Português (Brasil)",

        // Navigation
        nav_home: "Início",
        nav_about: "Sobre",
        nav_github: "GitHub",
        nav_download: "Baixar",
        nav_theme: "Tema",

        // Hero Section
        hero_tagline: "Código Aberto • Sem Fins Lucrativos • Local First • Seus Dados, Suas Regras",
        hero_title: "Lince",
        hero_subtitle: "Uma ferramenta poderosa de código aberto para registro, interconexão e automação de Necessidades e Contribuições. Feita pela comunidade, para a comunidade — sempre gratuita, sempre sua.",
        hero_download: "Baixar Lince",
        hero_docs: "Documentação",
        hero_learn_more: "Saiba Mais",

        // Index Page - Open Source Section
        opensource_title: "Código Aberto, Sem Fins Lucrativos, Sempre Gratuito",
        opensource_desc: "Lince é construído sobre princípios de transparência, propriedade comunitária e liberdade digital. Sem capital de risco, sem agendas ocultas — apenas software que serve você.",
        opensource_free_title: "100% Gratuito & Código Aberto",
        opensource_free_desc: "Cada linha de código está aberta para inspeção, modificação e redistribuição. Sem planos premium, sem paywalls, sem limitações artificiais. O código-fonte completo está disponível sob uma licença permissiva.",
        opensource_community_title: "Guiado pela Comunidade",
        opensource_community_desc: "O desenvolvimento é guiado pelas necessidades dos usuários, não por margens de lucro. Funcionalidades são priorizadas com base no feedback da comunidade e casos de uso reais. Todos podem contribuir com código, documentação ou ideias.",
        opensource_privacy_title: "Privacidade por Design",
        opensource_privacy_desc: "Seus dados ficam no seu dispositivo. Sem contas na nuvem, sem telemetria, sem rastreamento. Lince funciona totalmente offline. Você é dono dos seus dados — exporte, faça backup, delete. Controle total.",

        // Index Page - Goal Summary
        goal_title: "O que é o Lince?",
        goal_desc: "Em essência, Lince é um banco de dados pessoal para gerenciar qualquer coisa através da perspectiva de Necessidades e Contribuições.",
        goal_registry_title: "Registro Pessoal",
        goal_registry_desc: "Crie um banco de dados SQLite local (seu 'DNA') que armazena tudo: tarefas, hábitos, inventários, projetos. Seu centro de comando pessoal, armazenado como um único arquivo portátil.",
        goal_needs_title: "Necessidades & Contribuições",
        goal_needs_desc: "Um conceito simples mas poderoso: quantidades negativas são Necessidades (-2 Maçãs significa que você precisa de 2), positivas são Contribuições. Este modelo alimenta rastreamento, automação e trocas.",
        goal_automation_title: "Automação Inteligente",
        goal_automation_desc: "Defina necessidades recorrentes, acione comandos do sistema com base em estados, automatize rotinas diárias. Do rastreamento de hábitos à automação do computador — se é previsível, Lince pode lidar.",

        // Index Page - Links
        links_title: "Links Rápidos",
        link_source: "Código Fonte",
        link_download: "Baixar Última Versão",
        link_readme: "LEIA-ME",
        link_issues: "Reportar Problemas",
        link_discussions: "Discussões",
        link_contributing: "Guia de Contribuição",
        link_website: "Código do Site",
        link_org: "Organização GitHub",

        // About Page
        about_title: "Sobre o Lince",
        about_intro: "Lince é mais que software — é uma filosofia de propriedade de dados pessoais, desenvolvimento transparente e inovação guiada pela comunidade. Aqui está tudo que você precisa saber.",

        about_mission_title: "Nossa Missão",
        about_mission_text: "Acreditamos que ferramentas de produtividade pessoal devem pertencer aos usuários, não às corporações. Lince existe para fornecer uma alternativa poderosa, flexível e completamente gratuita a gerenciadores de tarefas, rastreadores de hábitos e ferramentas de automação proprietários. Estamos comprometidos a: nunca cobrar por funcionalidades principais, nunca vender dados de usuários (não coletamos nenhum), manter o desenvolvimento aberto e transparente, empoderar usuários com controle total sobre suas vidas digitais. Isso não é um negócio — é um projeto comunitário. Somos financiados por doações e contribuições voluntárias, não por investidores buscando retornos.",

        about_registry_title: "O Registro DNA",
        about_registry_text: "Quando você inicia o Lince pela primeira vez, ele cria um banco de dados pessoal armazenado localmente no seu sistema. Em sistemas UNIX (Linux, macOS), está localizado em ~/.config/lince/lince.db. Este banco de dados SQLite é seu 'DNA' — uma estrutura de dados flexível que pode ser moldada para representar virtualmente qualquer coisa. Tarefas, hábitos, inventários, projetos, transações econômicas, regras de automação — seu DNA se adapta ao seu caso de uso. A beleza do SQLite é a portabilidade: todo seu sistema pessoal é um único arquivo que você pode fazer backup, sincronizar, copiar ou compartilhar. Sem lock-in na nuvem, sem conta necessária. Seus dados viajam com você.",

        about_needs_title: "O Modelo de Necessidades & Contribuições",
        about_needs_text: "Todo comportamento no Lince se reduz a um conceito simples: Quantidade. Valores negativos representam Necessidades — coisas que você deve adquirir ou realizar. Valores positivos representam Contribuições — coisas que você tem ou completou. Precisa de 2 maçãs? Isso é -2 Maçãs no seu DNA. Comeu uma? Incremente 1 para -1 Maçãs. Este modelo elegantemente simples unifica: Gerenciamento de tarefas (Necessidade = -1 'Completar relatório'), Rastreamento de hábitos (Reset diário para -1 'Exercício'), Gerenciamento de inventário (Necessidade = -3 'Galões de leite'), Rastreamento de recursos (Contribuição = +500 'Dólares do orçamento'). Tudo é um número. Simples, mas incrivelmente poderoso.",

        about_automation_title: "Automação & Gatilhos",
        about_automation_text: "Se há previsibilidade na sua vida, Lince pode automatizar. Frequências Fixas ajustam automaticamente quantidades em um cronograma — resete seu hábito 'Estudar Espanhol' para -1 toda manhã, decremente 'Dias até o prazo' a cada 24 horas. Gatilhos de Estado executam comandos do sistema quando condições são atendidas — quando 'Modo Foco' igual a -1, execute um script para bloquear sites distrativos; quando 'Backup Necessário' atingir -7, acione sua rotina de backup. A camada de automação conecta seus dados pessoais às capacidades do seu computador. Combinado com scripts shell, isso se torna incrivelmente poderoso: notificações, controle de aplicações, manutenção do sistema, fluxos de trabalho personalizados. O limite é sua imaginação.",

        about_dna_title: "Padrões de DNA Compartilháveis",
        about_dna_text: "Grandes sistemas organizacionais frequentemente morrem com seus criadores. Conhecimento se perde, fluxos de trabalho são esquecidos. Lince resolve isso com padrões de DNA compartilháveis — templates de banco de dados pré-construídos para atividades comuns. Importe um DNA de rotina matinal para começar seu dia. Carregue um template de gerenciamento de projeto com dependências de tarefas. Obtenha um sistema de inventário doméstico com itens comuns pré-configurados. Compartilhe seu DNA de produtividade personalizado com a comunidade. Padrões permitem que você comece de sistemas comprovados ao invés de construir do zero. Conforme a comunidade cresce, a biblioteca de padrões compartilhados também cresce.",

        about_economic_title: "Transações Econômicas",
        about_economic_text: "O modelo de Necessidades/Contribuições se estende naturalmente à economia. Comércio é simplesmente a transferência de quantidades entre partes. Modele compras, vendas, débitos e créditos. Rastreie inventário em múltiplas localizações. Gerencie recursos compartilhados em residências ou organizações. Lide com transações multipartes com responsabilidade clara. Lince não é software de contabilidade, mas pode modelar relações econômicas quando você precisar.",

        about_tech_title: "Fundação Técnica",
        about_tech_text: "Lince é construído com Rust, escolhido por desempenho, confiabilidade e segurança de memória. As garantias em tempo de compilação previnem classes inteiras de bugs comuns em outras linguagens. Dados vivem em SQLite, o banco de dados mais implantado do mundo. Testado em batalha, leve e completamente autocontido — nenhum servidor de banco de dados necessário. A arquitetura é intencionalmente simples: um binário local, um banco de dados local, processamento local. Sem microserviços, sem containers, sem dependências de nuvem. Instale e execute. Funciona offline para sempre. A base de código é completamente documentada e acolhedora para contribuidores. Seja corrigindo bugs, adicionando funcionalidades ou aprendendo Rust, há um lugar para você.",

        about_philosophy_title: "Nossa Filosofia",
        about_philosophy_text: "Software deve servir usuários, não explorá-los. Ferramentas de produtividade não devem requerer assinaturas, não devem coletar dados, não devem prender você em ecossistemas. Construímos Lince porque queremos usá-lo nós mesmos. Cada funcionalidade existe porque uma pessoa real precisou. Cada decisão de design prioriza a liberdade do usuário. Este é software feito por pessoas, para pessoas. Junte-se a nós na construção de algo diferente. Algo que te respeita. Algo que é verdadeiramente seu.",

        about_get_started: "Comece com o Lince",
        about_download: "Baixar Última Versão",
        about_read_docs: "Ler a Documentação",
        about_quick_start: "Guia de Início Rápido",
        about_join_community: "Junte-se à Comunidade",

        // Footer
        footer_resources: "Recursos",
        footer_documentation: "Documentação",
        footer_downloads: "Downloads",
        footer_issues: "Rastreador de Problemas",
        footer_community: "Comunidade",
        footer_org: "Organização GitHub",
        footer_contributing: "Contribuindo",
        footer_legal: "Legal",
        footer_license: "Licença",
        footer_copyright: "Copyleft 2023-2026 Lince. Feito com amor e Rust.",
    });

    // ============================================================
    // MANDARIN CHINESE (Simplified)
    // ============================================================
    map.insert("zh", Translations {
        lang_code: "zh",
        lang_name: "中文",

        // Navigation
        nav_home: "首页",
        nav_about: "关于",
        nav_github: "GitHub",
        nav_download: "下载",
        nav_theme: "主题",

        // Hero Section
        hero_tagline: "开源 • 非营利 • 本地优先 • 您的数据，您做主",
        hero_title: "Lince",
        hero_subtitle: "一个强大的开源工具，用于需求和贡献的注册、互联和自动化。由社区构建，为社区服务——永远免费，永远属于您。",
        hero_download: "下载 Lince",
        hero_docs: "文档",
        hero_learn_more: "了解更多",

        // Index Page - Open Source Section
        opensource_title: "开源、非营利、永久免费",
        opensource_desc: "Lince 基于透明、社区所有权和数字自由的原则构建。没有风险投资，没有隐藏议程——只有为您服务的软件。",
        opensource_free_title: "100% 免费开源",
        opensource_free_desc: "每一行代码都可供检查、修改和再分发。没有高级层，没有付费墙，没有人为限制。完整源代码在宽松许可下提供。",
        opensource_community_title: "社区驱动",
        opensource_community_desc: "开发由用户需求引导，而非利润率。功能根据社区反馈和实际用例确定优先级。每个人都可以贡献代码、文档或想法。",
        opensource_privacy_title: "隐私设计",
        opensource_privacy_desc: "您的数据保留在您的设备上。无需云账户，无遥测，无追踪。Lince 完全离线工作。您拥有您的数据——导出、备份、删除。完全控制。",

        // Index Page - Goal Summary
        goal_title: "什么是 Lince？",
        goal_desc: "本质上，Lince 是一个个人数据库，通过需求和贡献的视角管理任何事物。",
        goal_registry_title: "个人注册表",
        goal_registry_desc: "创建一个本地 SQLite 数据库（您的 DNA），存储一切：任务、习惯、库存、项目。您的个人指挥中心，存储为单个便携文件。",
        goal_needs_title: "需求与贡献",
        goal_needs_desc: "一个简单但强大的概念：负数是需求（-2 苹果意味着您需要 2 个），正数是贡献。这个模型支持跟踪、自动化和交易。",
        goal_automation_title: "智能自动化",
        goal_automation_desc: "设置重复需求，根据状态触发系统命令，自动化日常事务。从习惯追踪到电脑自动化——如果可预测，Lince 都能处理。",

        // Index Page - Links
        links_title: "快速链接",
        link_source: "源代码",
        link_download: "下载最新版",
        link_readme: "自述文件",
        link_issues: "报告问题",
        link_discussions: "讨论",
        link_contributing: "贡献指南",
        link_website: "网站源码",
        link_org: "GitHub 组织",

        // About Page
        about_title: "关于 Lince",
        about_intro: "Lince 不仅仅是软件——它是个人数据所有权、透明开发和社区驱动创新的理念。以下是您需要了解的一切。",

        about_mission_title: "我们的使命",
        about_mission_text: "我们相信个人生产力工具应该属于用户，而不是公司。Lince 的存在是为了提供一个强大、灵活、完全免费的替代品，取代专有的任务管理器、习惯追踪器和自动化工具。我们承诺：永不收取核心功能费用，永不出售用户数据（我们不收集任何数据），保持开发开放透明，赋予用户对数字生活的完全控制。这不是生意——这是社区项目。我们由捐赠和志愿贡献资助，而非寻求回报的投资者。",

        about_registry_title: "DNA 注册表",
        about_registry_text: "当您首次启动 Lince 时，它会在您的系统上本地创建一个个人数据库。在 UNIX 系统（Linux、macOS）上，它位于 ~/.config/lince/lince.db。这个 SQLite 数据库是您的 DNA——一个灵活的数据结构，可以塑造成几乎任何东西。任务、习惯、库存、项目、经济交易、自动化规则——您的 DNA 适应您的用例。SQLite 的美妙之处在于便携性：您整个个人系统是一个可以备份、同步、复制或共享的单一文件。无云锁定，无需账户。您的数据随您而行。",

        about_needs_title: "需求与贡献模型",
        about_needs_text: "Lince 中的每个行为都归结为一个简单概念：数量。负值代表需求——您必须获取或完成的事物。正值代表贡献——您拥有或已完成的事物。需要 2 个苹果？那就是您 DNA 中的 -2 苹果。吃了一个？增加 1 变成 -1 苹果。这个优雅简单的模型统一了：任务管理（需求 = -1 完成报告），习惯追踪（每日重置为 -1 锻炼），库存管理（需求 = -3 加仑牛奶），资源追踪（贡献 = +500 预算美元）。一切都是数字。简单，但极其强大。",

        about_automation_title: "自动化与触发器",
        about_automation_text: "如果您的生活中有可预测性，Lince 可以自动化它。固定频率按计划自动调整数量——每天早上将您的学习西班牙语习惯重置为 -1，每 24 小时递减距离截止日期的天数。状态触发器在满足条件时执行系统命令——当专注模式等于 -1 时，运行脚本阻止分心网站；当需要备份达到 -7 时，触发您的备份例程。自动化层将您的个人数据连接到计算机的功能。结合 shell 脚本，这变得极其强大：通知、应用程序控制、系统维护、自定义工作流。限制只是您的想象力。",

        about_dna_title: "可共享的 DNA 模式",
        about_dna_text: "伟大的组织系统经常随创建者消亡。知识丢失，工作流程被遗忘。Lince 通过可共享的 DNA 模式解决这个问题——用于常见活动的预建数据库模板。导入早晨例程 DNA 开始您的一天。加载带有任务依赖的项目管理模板。获取预配置常见物品的家庭库存系统。与社区分享您自定义的生产力 DNA。模式让您从经过验证的系统开始，而不是从零构建。随着社区增长，共享模式库也在增长。",

        about_economic_title: "经济交易",
        about_economic_text: "需求/贡献模型自然延伸到经济领域。交易只是各方之间数量的转移。建模购买、销售、债务和信贷。跟踪多个位置的库存。管理家庭或组织中的共享资源。处理责任明确的多方交易。Lince 不是会计软件，但在您需要时可以建模经济关系。",

        about_tech_title: "技术基础",
        about_tech_text: "Lince 使用 Rust 构建，选择它是因为性能、可靠性和内存安全。编译时保证防止了其他语言中常见的整类 bug。数据存储在 SQLite 中，这是世界上部署最广泛的数据库。经过实战检验，轻量级，完全独立——不需要数据库服务器。架构故意保持简单：本地二进制文件，本地数据库，本地处理。没有微服务，没有容器，没有云依赖。安装即运行。永久离线工作。代码库有完整文档，欢迎贡献者。无论您是修复 bug、添加功能还是学习 Rust，都有您的位置。",

        about_philosophy_title: "我们的理念",
        about_philosophy_text: "软件应该服务用户，而不是剥削他们。生产力工具不应该需要订阅，不应该收集数据，不应该把您锁在生态系统中。我们构建 Lince 是因为我们自己想使用它。每个功能的存在都是因为真实的人需要它。每个设计决策都优先考虑用户自由。这是由人为人制作的软件。加入我们，一起构建不同的东西。尊重您的东西。真正属于您的东西。",

        about_get_started: "开始使用 Lince",
        about_download: "下载最新版本",
        about_read_docs: "阅读文档",
        about_quick_start: "快速入门指南",
        about_join_community: "加入社区",

        // Footer
        footer_resources: "资源",
        footer_documentation: "文档",
        footer_downloads: "下载",
        footer_issues: "问题追踪",
        footer_community: "社区",
        footer_org: "GitHub 组织",
        footer_contributing: "贡献",
        footer_legal: "法律",
        footer_license: "许可证",
        footer_copyright: "Copyleft 2023-2026 Lince。用爱和 Rust 构建。",
    });

    map
}
