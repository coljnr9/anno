#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use dioxus_helmet::Helmet;
use dioxus_router::prelude::*;
use log::Level;

use rss::Channel;

mod clipboard;
use clipboard::use_clipboard;

#[derive(PartialEq, Clone, Props)]
pub struct Article {
    pub id: usize,
    pub author: String,
    pub title: String,
    pub preview: String,
}

pub async fn get_rss_channel() -> Channel {
    if let Some(c) = CHANNEL() {
        return c;
    }
    let content = reqwest::get("https://feeds.simplecast.com/qm_9xx0g")
        .await
        .expect("Error retrieving feed")
        .bytes()
        .await
        .expect("Error getting bytes");
    let channel = Channel::read_from(&content[..]).expect("Error creating channel");
    channel
}

#[derive(Clone)]
struct AppState {
    cached_channel: Option<Channel>,
}

fn main() {
    let log_config = wasm_logger::Config::new(Level::Info);
    wasm_logger::init(log_config);
    launch(App);
}

#[component]
fn HeadElements(path: String) -> Element {
    rsx! {
        Helmet {
            link { rel: "preconnect", href: "https://fonts.googleapis.com" }
            link {
                crossorigin: "false",
                href: "https://fonts.gstatic.com",
                rel: "preconnect"
            }
            link {
                href: "https://fonts.googleapis.com/css2?family=Titillium+Web:ital,wght@0,400;0,700;1,400;1,700&display=swap",
                rel: "stylesheet"
            }
        }
    }
}

#[component]
fn BeginSubmit() -> Element {
    // TODO(coljnr9) Deal with the mess that is clipboard on web.

    rsx! {
        Link { to: Route::Submission {}, class: "begin-submit",
            div {
                p { "Paste URL" }
            }
        }
    }
}

fn Submission() -> Element {
    rsx! {
        ConfirmSubmission {}
        ArticleSubmissionPreview {}
        CancelSubmission {}
    }
}
fn GoToArchive() -> Element {
    rsx! {
        Link { to: Route::Archive {}, class: "go-to-archive",
            div {
                p { "Archive" }
            }
        }
    }
}

fn GoHome() -> Element {
    rsx! {
        Link { to: Route::Home {}, class: "go-home",
            div {
                p { "Home" }
            }
        }
    }
}
#[component]
fn AuthorLink(author: ReadOnlySignal<Option<Channel>>, author_id: usize) -> Element {
    match author() {
        Some(channel) => {
            rsx! {
                div { class: "author-link-region",
                    Link {
                        to: Route::AuthorPage { author_id },
                        onclick: |e: MouseEvent| {
                            e.stop_propagation();
                        },
                        div { "PLACEHOLDER: {channel.link}" }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "author-link-region",
                    Link {
                        to: Route::AuthorPage { author_id },
                        onclick: |e: MouseEvent| {
                            e.stop_propagation();
                        },
                        div { "Loading..." }
                    }
                }
            }
        }
    }
}

#[component]
fn ConfirmSubmission() -> Element {
    rsx! {
        div { class: "confirm-submission-region", "Confirm" }
    }
}

#[component]
fn CancelSubmission() -> Element {
    rsx! {
        Link { class: "cancel-submission-region", to: Route::Home {},

            div { { "Cancel" } }
        }
    }
}

async fn load_webpage(url: String) -> String {
    let page = reqwest::get(url).await.expect("Error fetching webpage");
    page.text().await.expect("Error retrieving text")
}
#[component]
fn ArticleSubmissionPreview() -> Element {
    let cb = use_resource(move || use_clipboard());
    match &*cb.read_unchecked() {
        Some(d) => {
            rsx! {
                div { class: "article-submission-preview",
                    iframe { src: d.clone(), height: "100%" }
                }
            }
        }
        // let d_ = d.clone();
        // let webpage = use_resource(move || load_webpage(d_.clone()));
        // match &*webpage.read_unchecked() {
        //     Some(wb) => {
        //         rsx! {
        //             div { class: "article-submission-preview", iframe {
        //                 src: d
        //             } }
        //         }
        //     }
        //     None => {
        //         rsx! { div { class: "article-submission-preview", "Loading..." }
        //         }
        //     }
        // }
        // }
        None => {
            rsx! {
                div { class: "article-submission-preview", "No URL" }
            }
        }
    }
}
#[component]
fn ArticleTitle(title: ReadOnlySignal<Option<Channel>>, article_id: usize) -> Element {
    match title() {
        Some(c) => {
            rsx! {
                Link { to: Route::ArticlePage { article_id },
                    div { class: "article-title-region", "PLACEHOLDER: {c.title}" }
                }
            }
        }
        None => {
            rsx! {
                div { class: "article-title-region", "loading" }
            }
        }
    }
}

#[component]
fn ArticlePreview(article: ReadOnlySignal<Article>) -> Element {
    let Article {
        id,
        author,
        title,
        preview,
    } = &*article.read();

    let channel = use_resource(|| get_rss_channel());

    match &*channel.read_unchecked() {
        Some(c) => {
            if CHANNEL().is_none() {
                *CHANNEL.write() = Some(c.clone())
            }
        }
        None => {}
    };

    rsx! {
        div { class: "article-preview",
            ArticleTitle { title: CHANNEL(), article_id: 1 }
            AuthorLink { author: CHANNEL(), author_id: 1 }
        }
    }
}

#[component]
fn ArticlePage(article_id: usize) -> Element {
    // TODO(coljnr9) Fetch article data
    let article1 = Article {
        id: article_id,
        author: "Author".to_string(),
        title: "Title".to_string(),
        preview: "Preview text preview text preview text".to_string(),
    };

    rsx! {
        Link { to: Route::Home {}, { "Home" } }
        ArticleReadingView { article: article1 }
        RequeueArticle {}
    }
}
#[component]
fn ArticleReadingView(article: Article) -> Element {
    let Article {
        id, author, title, ..
    } = article;
    rsx! {
        p { "Article {id}" }
        p { "{author} - {title}" }
    }
}

#[component]
fn RequeueArticle() -> Element {
    rsx! {
        p { "Requeue" }
    }
}

#[component]
fn AuthorPage(author_id: usize) -> Element {
    rsx! {
        Link { to: Route::Home {}, { "Home" } }
        { "Author Page" },
        h3 { "Author {author_id}" }
    }
}
#[component]
fn Home() -> Element {
    let article1 = Article {
        id: 1,
        author: "Author".to_string(),
        title: "Title".to_string(),
        preview: "Preview text preview text preview text".to_string(),
    };
    rsx! {
        BeginSubmit {}
        ArticlePreview { article: article1.clone() }
        ArticlePreview { article: article1.clone() }
        ArticlePreview { article: article1.clone() }
        ArticlePreview { article: article1.clone() }
        ArticlePreview { article: article1.clone() }
        GoToArchive {}
    }
}

#[component]
fn ArchiveArticleList() -> Element {
    let article1 = Article {
        id: 1,
        author: "Author".to_string(),
        title: "Title".to_string(),
        preview: "Preview text preview text preview text".to_string(),
    };
    rsx! {
        div { class: "article-archive-list",
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
            ArticlePreview { article: article1.clone() }
        }
    }
}
#[component]
fn Archive() -> Element {
    rsx! {
        GoHome {}
        ArchiveArticleList {}
    }
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/submit")]
    Submission {},

    #[route("/archive")]
    Archive {},

    #[route("/article/:article_id")]
    ArticlePage { article_id: usize },

    #[route("/author/:author_id")]
    AuthorPage { author_id: usize },
}

static CHANNEL: GlobalSignal<Option<Channel>> = Signal::global(|| None);
static CLIPBOARD_DATA: GlobalSignal<Option<String>> = Signal::global(|| None);

pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        HeadElements { path: "" }
        Router::<Route> {}
    }
}
