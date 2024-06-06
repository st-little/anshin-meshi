#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::{icons::io_icons::{IoLogoTwitter, IoOpen, IoSearch}, Icon};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use anyhow::Result;

mod env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    /** 届出番号 */
    notificationNumber: String,
    /** 商品名 */
    productName: String,
    /** 届出者名 */
    notifierName: String,
    /** 表示しようとする機能性 */
    functionalityToDisplay: String,
    /** ASCON 総合評価判定 */
    assessment: String,
    /** 論文採用の根拠/機能性エビデンスの総評 */
    generalReviewOfEvidence: String,
}

impl Data {
    fn new() -> Self {
        Self {
            notificationNumber: "".to_string(),
            productName: "".to_string(),
            notifierName: "".to_string(),
            functionalityToDisplay: "".to_string(),
            assessment: "".to_string(),
            generalReviewOfEvidence: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    searchInput: String,
    topNavbarBurgerActive: bool,
    topNavbarBurgerClass: String,
    topNavbarMenuClass: String,
    detailModalActive: bool,
    detailModalClass: String,
    detailModalData: Data,
    aboutModalActive: bool,
    aboutModalClass: String,
    termsOfUseModalActive: bool,
    termsOfUseModalClass: String,
    privacyPolicyModalActive: bool,
    privacyPolicyModalClass: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            searchInput: "".to_string(),
            topNavbarBurgerActive: false,
            topNavbarBurgerClass: "navbar-burger".to_string(),
            topNavbarMenuClass: "navbar-menu".to_string(),
            detailModalActive: false,
            detailModalClass: "modal".to_string(),
            detailModalData: Data::new(),
            aboutModalActive: false,
            aboutModalClass: "modal".to_string(),
            termsOfUseModalActive: false,
            termsOfUseModalClass: "modal".to_string(),
            privacyPolicyModalActive: false,
            privacyPolicyModalClass: "modal".to_string(),
        }
    }
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppState::new()));
    let app_state = consume_context::<Signal<AppState>>();
    let data_resource = use_resource(move || get_data());

    rsx! {
        link { rel: "stylesheet", href: "styles/main.css" }
        GoogleAnalytics {}
        Header {}
        div { class: "container p-3",
            match &*data_resource.read_unchecked() {
                Some(Ok(items)) => {
                    log::info!("get data: {:?}", items);
                    let filtered_items = items.iter().filter(|item| item.productName.contains(&app_state.read().searchInput)).collect::<Vec<&Data>>();
                    rsx! {
                        div {
                            class: "table-container",
                            table {
                                class: "table is-fullwidth mt-3",
                                thead {
                                    tr {
                                        th { "商品名 一覧" }
                                    }
                                }
                                tbody {
                                    if filtered_items.len() == 0 {
                                        tr {
                                            td { "該当する商品がありません。" }
                                        }
                                    } else {
                                        for item in filtered_items {
                                            TableRow {item: item.clone()}
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
                Some(Err(err)) => {
                    log::error!("Error: {:?}", err);
                    rsx! {
                        article {
                            class: "message is-danger",
                            div {
                                class: "message-body",
                                "データの取得に失敗しました。再読み込みをお試し下さい。"
                            }
                        }
                    }
                }
                None => {
                    rsx! {Loading {}}
                }
            }
        }
        DetailModal {}
        AboutModal {}
        TermsOfUseModal {}
        PrivacyPolicyModal {}
    }
}

#[component]
fn GoogleAnalytics() -> Element {
    rsx! {
        script {
            r#async: true,
            src: "https://www.googletagmanager.com/gtag/js?id={env::APP_GA_TRACKING_ID}"
        }
        script {
            r#"
            window.dataLayer = window.dataLayer || [];
            function gtag(){{dataLayer.push(arguments);}}
            gtag('js', new Date());
            gtag('config', '{env::APP_GA_TRACKING_ID}');
            "#
        }
    }
}

#[component]
fn Header() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        header {
            nav { id: "top-navbar", class: "navbar",
                div { class: "navbar-brand",
                    a { href: "#", class: "navbar-item header-title", "アンシンめし"}
                    a {
                        id: "top-navbar-burger",
                        class: "{app_state.read().topNavbarBurgerClass}",
                        "aria-expanded": "false",
                        "data-target": "top-navbar-menu",
                        onclick: move |_| {
                            if app_state.read().topNavbarBurgerActive {
                                app_state.write().topNavbarBurgerActive = false;
                                app_state.write().topNavbarBurgerClass = "navbar-burger".to_string();
                                app_state.write().topNavbarMenuClass = "navbar-menu".to_string();
                            } else {
                                app_state.write().topNavbarBurgerActive = true;
                                app_state
                                    .write()
                                    .topNavbarBurgerClass = "navbar-burger is-active".to_string();
                                app_state.write().topNavbarMenuClass = "navbar-menu is-active".to_string();
                            }
                        },
                        span { "aria-hidden": "true" }
                        span { "aria-hidden": "true" }
                        span { "aria-hidden": "true" }
                        span { "aria-hidden": "true" }
                    }
                }
                div {
                    id: "top-navbar-menu",
                    class: "{app_state.read().topNavbarMenuClass}",
                    div { class: "navbar-start" }
                    div { class: "navbar-end",
                        div { class: "navbar-item",
                            div { class: "buttons",
                                a {
                                    href: "#",
                                    class: "Header-link",
                                    "data-target": "about-modal",
                                    onclick: move |_| {
                                        if app_state.read().aboutModalActive {
                                            app_state.write().aboutModalActive = false;
                                            app_state.write().aboutModalClass = "modal".to_string();
                                        } else {
                                            app_state.write().aboutModalActive = true;
                                            app_state.write().aboutModalClass = "modal is-active".to_string();
                                        }
                                    },
                                    "このサイトについて"
                                }
                                a {
                                    href: "#",
                                    class: "Header-link",
                                    "data-target": "terms-of-use-modal",
                                    onclick: move |_| {
                                        if app_state.read().termsOfUseModalActive {
                                            app_state.write().termsOfUseModalActive = false;
                                            app_state.write().termsOfUseModalClass = "modal".to_string();
                                        } else {
                                            app_state.write().termsOfUseModalActive = true;
                                            app_state.write().termsOfUseModalClass = "modal is-active".to_string();
                                        }
                                    },
                                    "利用規約"
                                }
                                a {
                                    href: "#",
                                    class: "Header-link",
                                    "data-target": "privacy-policy-modal",
                                    onclick: move |_| {
                                        if app_state.read().privacyPolicyModalActive {
                                            app_state.write().privacyPolicyModalActive = false;
                                            app_state.write().privacyPolicyModalClass = "modal".to_string();
                                        } else {
                                            app_state.write().privacyPolicyModalActive = true;
                                            app_state.write().privacyPolicyModalClass = "modal is-active".to_string();
                                        }
                                    },

                                    "個人情報保護方針"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "container px-3 pt-3",
                div { class: "control has-icons-left",
                    input {
                        class: "input is-medium",
                        r#type: "text",
                        placeholder: "商品名を入力してください",
                        oninput: move |event| app_state.write().searchInput = event.value()
                    }
                    span { class: "icon is-medium is-left", Icon { width: 24, height: 24, icon: IoSearch } }
                }
            }
        }
    }
}

#[component]
fn TableRow(item: ReadOnlySignal<Data>) -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        tr { onclick: move |_| {
                log::info!("click: {:?}", item());
                if app_state.read().detailModalActive {
                    app_state.write().detailModalActive = false;
                    app_state.write().detailModalClass = "modal".to_string();
                    app_state.write().detailModalData = Data::new();
                } else {
                    app_state.write().detailModalActive = true;
                    app_state.write().detailModalClass = "modal is-active".to_string();
                    app_state.write().detailModalData = item();
                }
            },
            td { "{item().productName}" }
        }
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
        div { class: "loader-wrapper", span { class: "loader" } }
    }

}

#[component]
fn DetailModal() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div { id: "detail-modal", class: "{app_state.read().detailModalClass}",
            div { class: "modal-background" }
            div { class: "modal-card p-4",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "機能性表示評価成績" }
                    button {
                        class: "modal-close is-large",
                        "aria-label": "close",
                        onclick: move |_| {
                            app_state.write().detailModalActive = false;
                            app_state.write().detailModalClass = "modal".to_string();
                        }
                    }
                }
                section { class: "modal-card-body", DetailContent {} }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button {
                            class: "button",
                            onclick: move |_| {
                                app_state.write().detailModalActive = false;
                                app_state.write().detailModalClass = "modal".to_string();
                            },
                            "閉じる"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DetailContent() -> Element {
    let app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div { class: "content",
            h3 { "届出番号" }
            p { "{app_state.read().detailModalData.notificationNumber}" }
            h3 { "商品名" }
            p { "{app_state.read().detailModalData.productName}" }
            h3 { "届出者名" }
            p { "{app_state.read().detailModalData.notifierName}" }
            h3 { "表示しようとする機能性" }
            p { "{app_state.read().detailModalData.functionalityToDisplay}" }
            h3 { "ASCON 総合評価判定" }
            p { "{app_state.read().detailModalData.assessment}" }
            h3 { "論文採用の根拠/機能性エビデンスの総評" }
            p { "{app_state.read().detailModalData.generalReviewOfEvidence}" }
        }
    }
}

#[component]
fn AboutModal() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div { id: "about-modal", class: "{app_state.read().aboutModalClass}",
            div { class: "modal-background" }
            div { class: "modal-card p-4",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "このサイトについて" }
                    button {
                        class: "modal-close is-large",
                        "aria-label": "close",
                        onclick: move |_| {
                            app_state.write().aboutModalActive = false;
                            app_state.write().aboutModalClass = "modal".to_string();
                        }
                    }
                }
                section { class: "modal-card-body", AboutContent {} }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button {
                            class: "button",
                            onclick: move |_| {
                                app_state.write().aboutModalActive = false;
                                app_state.write().aboutModalClass = "modal".to_string();
                            },
                            "閉じる"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AboutContent() -> Element {
    rsx! {
        div { class: "content",
            h3 { "目的" }
            p {
                "このサイトは 「ASCON科学者委員会」 が公開している 機能性表示評価成績 を商品名から検索し閲覧することが目的です。"
            }
            article { class: "message is-warning",
                div { class: "message-body",
                    "このサイトは非公式のものであり、「ASCON科学者委員会」 とは一切関係ありません。"
                }
            }
            h3 { "使い方" }
            ol { class: "type='1'",
                li {
                    "サイトを開いたらデータの取得が完了するのを待ちます。データの取得が完了すると商品名一覧が表示されます。"
                }
                li { "商品名の検索窓に検索したい商品名を入力します。" }
                li {
                    "商品名一覧から商品名をタップすると機能性表示評価成績が表示されます。"
                }
            }
            h3 { "サイト情報" }
            ul {
                li { "サイト名: アンシンめし"}
                li { "バージョン: {env::APP_VERSION}" }
                li {
                    "Repository: "
                    a { href: env::APP_GITHUB_URL, target: "_blank", "GitHub" }
                    span { class: "icon is-small mr-2", Icon { width: 16, height: 16, icon: IoOpen } }
                }
                li { "クレジット: © 2020 st-little" }
    
            }
            h3 {
                "サードパーティクレジット"
            }
            dl {
                dt {
                    "機能性表示評価成績"
                }
                dd {
                    "© ASCON科学者委員会"
                }
                dd {
                    "http://ascon.bz/"
                }
                dt {
                    "になロマン"
                }
                dd {
                    "© 213ちゃん"
                }
                dd {
                    "https://213chan.booth.pm/items/5570965"
                }
    
            }
            h3 { "更新履歴" }
            p { "0.1.0: ベータ版リリース" }
        }
    }
}

#[component]
fn TermsOfUseModal() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div {
            id: "terms-of-use-modal",
            class: "{app_state.read().termsOfUseModalClass}",
            div { class: "modal-background" }
            div { class: "modal-card p-4",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "利用規約" }
                    button {
                        class: "modal-close is-large",
                        "aria-label": "close",
                        onclick: move |_| {
                            app_state.write().termsOfUseModalActive = false;
                            app_state.write().termsOfUseModalClass = "modal".to_string();
                        }
                    }
                }
                section { class: "modal-card-body", TermsOfUseContent {} }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button {
                            class: "button",
                            onclick: move |_| {
                                app_state.write().termsOfUseModalActive = false;
                                app_state.write().termsOfUseModalClass = "modal".to_string();
                            },
                            "閉じる"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TermsOfUseContent() -> Element {
    rsx! {
        div { class: "content",
            h3 { "1. 受諾" }
            p {
                "1.1 このウェブサービス（以下、「本サービス」といいます）を利用する場合、ユーザーは本利用規約に同意したものとみなされます。本サービスの利用は、本規約のすべての条件、および変更に同意することを含みます。"
            }
            h3 { "2. 定義" }
            p {
                "2.1 「本サービス」とは、本規約に基づき提供されるウェブサービスを指します。"
            }
            p {
                "2.2 「ユーザー」とは、本サービスを利用する個人または法人を指します。"
            }
            h3 { "3. サービスの提供" }
            p {
                "3.1 ユーザーは、本サービスの提供にあたり、合理的な努力を行いますが、本サービスの中断、遅延、またはエラーが生じる可能性があることを理解し、同意します。"
            }
            p {
                "3.2 ユーザーは、事前の通知なしに、本サービスの一部または全部を変更、中断、または終了する権利を留保します。"
            }
            h3 { "4. 利用条件" }
            p {
                "4.1 ユーザーは、本サービスを利用する際に、全ての適用される法律および規制を遵守する必要があります。"
            }
            p {
                "4.2 ユーザーは、本サービスを不正に使用し、または他のユーザーの利用を妨害する行為を行ってはなりません。"
            }
            p {
                "4.3 ユーザーは、本サービスを使用する際に、他のユーザーや本サービスの権利を侵害するような情報を提供してはなりません。"
            }
            h3 { "5. 個人情報の取り扱い" }
            p {
                "5.1 個人情報の収集、使用、および開示に関しては、個人情報保護方針が適用されます。"
            }
            h3 { "6. 責任の制限" }
            p {
                "6.1 当サービスの利用に関連して発生したいかなる損害についても、直接的、間接的、偶発的、特別、または重大な損害を含むがこれに限定されない、いかなる損害に対しても一切の責任を負いません。"
            }
            h3 { "7. 準拠法と管轄裁判所" }
            p { "7.1 本規約の解釈および適用は、日本法に従います。" }
            p {
                "7.2 本規約に関連するいかなる紛争も、東京地方裁判所を第一審の専属的な管轄裁判所とします。"
            }
        }
    }
}

#[component]
fn PrivacyPolicyModal() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();

    rsx! {
        div {
            id: "privacy-policy-modal",
            class: "{app_state.read().privacyPolicyModalClass}",
            div { class: "modal-background" }
            div { class: "modal-card p-4",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "個人情報保護方針" }
                    button {
                        class: "modal-close is-large",
                        "aria-label": "close",
                        onclick: move |_| {
                            app_state.write().privacyPolicyModalActive = false;
                            app_state.write().privacyPolicyModalClass = "modal".to_string();
                        }
                    }
                }
                section { class: "modal-card-body", PrivacyPolicyContent {} }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button {
                            class: "button",
                            onclick: move |_| {
                                app_state.write().privacyPolicyModalActive = false;
                                app_state.write().privacyPolicyModalClass = "modal".to_string();
                            },
                            "閉じる"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PrivacyPolicyContent() -> Element {
    rsx! {
        div { class: "content",
            p {
                "このウェブサイトは、Google Analytics を使用して、ウェブサイトのトラフィックとユーザーの行動に関する情報を収集しています。Google Analytics は Cookie を使用して、匿名の形式で情報を収集します。収集される情報には、ウェブサイトの利用者の IP アドレス、地理的位置、閲覧されたページ、利用されたブラウザやデバイスの種類などが含まれます。これらの情報は、個々のユーザーを特定するために使用されることはありません。"
            }
            p {
                "このウェブサイトは、Google Analytics の機能によって提供されるデータを収集、解析、報告するためにこれらの情報を使用します。これには、ウェブサイトの改善や、ユーザーのニーズに合わせたコンテンツの提供などが含まれます。"
            }
            p {
                "このウェブサイトを利用することにより、Google が収集したデータの処理に関して、Google の個人情報保護方針に同意したものとみなされます。Google の個人情報保護方針については、", a {
                    href: "https://policies.google.com/privacy",
                    target: "_blank",
                    "こちら",
                    span { class: "icon is-small mr-2", Icon { width: 16, height: 16, icon: IoOpen } }
                }
                "をご参照ください。"
            }
            p {
                "Cookie の使用に関する設定を変更したい場合は、ウェブブラウザの設定を変更して、Cookie の使用を管理することができます。ただし、Cookie の無効化または削除は、ウェブサイトの機能やサービスの一部を制限する可能性があります。"
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer has-text-centered",
            div { class: "container",
                div { class: "columns",
                    div { class: "column",
                        a {
                            class: "button",
                            href: "https://twitter.com/stlittle8",
                            span { class: "icon is-small mr-2", Icon { width: 16, height: 16, icon: IoLogoTwitter } }
                            "Twitter"
                        }
                    }
                }
                div { class: "has-text-centered", "© 2020 st-little" }
            }
        }
    }
}

async fn get_data() -> Result<Vec<Data>> {
    let url = format!("https://script.google.com/macros/s/{}/exec?v=v{}", env::APP_GAS_DEPLOYMENT_ID, env::APP_API_VERSION);
    let data = reqwest::get(url).await?.json::<Vec<Data>>().await?;
    Ok(data)
}
