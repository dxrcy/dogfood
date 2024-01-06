use ibex::prelude::*;
use ibex::{routes, ssg};

mod parse;
use parse::{Entry, Verdict};

const URL_ROOT: &str = "/dogfood/";

fn main() {
    let entries = parse::get_entries();

    let routes = routes! [
        (/)    => at_index(entries),
        (/404) => at_404(),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn at_index(entries: Vec<Entry>) -> Document {
    document! { [lang="en"]
        @use_base []
        div ."columns" {
            div ."short-list" {
                ul { [:for entry in &entries {
                    li { a [href=format!("#{}", name_to_id(&entry.name))] {
                        [&entry.name]
                    }}
                }]}
            }

            div ."long-list" {
                [:for entry in &entries {
                    @list_item [entry, &entries]
                }]
            }

            div ."side-panel" {
                hr/
                h2 { "In General" }
                p { "The main human foods not to feed a dog are:" }
                ul {
                    li { a [href="#chocolate"]
                        { "Chocolate" }}
                    li { a [href="#xylitol"]
                        { "Xylitol" } ~ "(Artificial sweetener)" }
                    li { a [href="#grapes"]
                        { "Grapes" }}
                    li { a [href="#onion-garlic-chives-leeks"]
                        { "Onions, garlic, ect" }}
                }
                div ."disclaimer" {
                    h2 { "Disclaimer:" }
                    p {
                        "Please read the sources listed before trusting the information given."
                        ~ "This website does not offer health advice."
                    }
                    p {
                        "Submit any issues"
                        ~ @smart_link [
                            "https://github.com/darccyy/dogfood/issues/new",
                            Some("here"),
                        ]
                    }
                }
            }

            button ."top-button" {
                a [href="#"] { "↑" }
            }
        }
    }
    .into()
}

fn list_item(entry: &Entry, entries: &[Entry]) -> View {
    let id = name_to_id(&entry.name);
    view! {
        article #[id] ."item" {
            hr/
            h2 ."name" {
                a [href=format!("#{}", id)] {
                    [&entry.name]
                    ~ "-" ~
                    i { [match &entry.verdict {
                        Verdict::Good  => "Yes!",
                        Verdict::Bad   => "NO!",
                        Verdict::Maybe => "Maybe...",
                    }] }
                }
            }

            [:if let Some(subtitle) = &entry.subtitle {
                h4 ."subtitle" { [subtitle] }
            }]

            p ."description" { [&entry.description] }

            div ."sources" {
                [:if !entry.sources.is_empty() {
                    p { i { "Sources:" } }
                    ul { [:for source in &entry.sources {
                        li { @smart_link [source, None] }
                    }] }
                } else {
                    p { b { "No sources!" } }
                }]
            }

            [:if let Some(review) = &entry.review {
                p ."review" {
                    i { "Critic's Review:" }
                    ~ [review]
                }
            }]

            div ."related" {
                details {
                    summary { "Related" }
                    ul { [:for other in entries {
                        [:if
                            other.name != entry.name
                            && do_lists_intersect(&other.tags, &entry.tags)
                         {
                            li { a [href=format!("#{}", name_to_id(&other.name))] {
                                [&other.name]
                            } }
                        }]
                    }] }
                }
            }
        }
    }
}

fn smart_link(link: &str, text: Option<&str>) -> View {
    // long version is for print
    view! {
        span ."smart-link" {
            [:if let Some(text) = text {
                span ."short" {
                    a [href=link] { [text] }
                }
                span ."long" [style="display:none"] {
                    span ."text" { [text] }
                    ":" ~
                    a [href=link] { [link] }
                }
            } else {
                span ."short" {
                    a [href=link] { [shorten_url(link)] }
                }
                span ."long" [style="display:none"] {
                    a [href=link] { [link] }
                }
            }]
        }
    }
}

fn at_404() -> Document {
    document! { [lang="en"]
        @use_base []
        div ."not-found" {
            h2 {
                "... a 404 Error? -"
                ~ i { "NO!!!" }
            }
            p { a [href=url!()] {
                "Back to the correct page?"
            }}
        }
    }
    .into()
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [Meta::new()
                .url(url!())
                .title("Can my dog eat...?")
                .desc("Which foods can dogs eat.")
                .image("static/icon.png")
                .author("darcy")
                .color("#a5b7cf")
            ]
            title { "Can my dog eat...?" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
            @ssg::use_autoreload []
        }

        h1 ."heading" {
            "Can my dog eat..."
        }
    }
}

/// Convert name into HTML #id attribute format, with limited character set
fn name_to_id(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|ch| {
            if matches!(ch, 'a'..='z'|'0'..='9'|'_'|'-'|'+') {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .replace("--", "-")
}

fn do_lists_intersect<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.iter().any(|item| b.contains(item))
}

fn shorten_url(link: &str) -> &str {
    let mut split_protocol = link.split("://");
    split_protocol.next();
    let Some(rest) = split_protocol.next() else {
        return link;
    };
    let Some(full_domain) = rest.split("/").next() else {
        return link;
    };
    if !full_domain.starts_with("www.") {
        return full_domain;
    }
    let Some(index) = full_domain.find('.') else {
        return full_domain;
    };
    let (_, domain) = full_domain.split_at(index + 1);
    if domain.is_empty() {
        return full_domain;
    }
    domain
}
