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

    ssg::quick_build(routes).unwrap();
    println!("All done!");
}

fn at_index(entries: Vec<Entry>) -> Document {
    view! {
        @use_base

        div [class="columns"] {
            div [class="short-list"] {
                ul { [:for entry in &entries {
                    li { a [href=format!("#{}", name_to_id(&entry.name))] {
                        [&entry.name]
                    }}
                }]}
            }

            div [class="long-list"] {
                [:for entry in &entries {
                    @list_item [entry, &entries]
                }]
            }

            div [class="side-panel"] {
                hr/
                h1 { "In General" }
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
                div [class="disclaimer"] {
                    h1 { "Disclaimer:" }
                    p {
                        "Please read the sources listed before trusting the information given."
                        ~ "This website does not offer health advice."
                    }
                    p {
                        "Submit any issues"
                        ~ @short_link [
                            "https://github.com/darccyy/dogfood/issues/new",
                            Some("here"),
                        ]
                    }
                }
            }

            a [class="top-button", href="#"] {
                button { "↑" }
            }
        }
    }
    .into()
}

fn list_item(entry: &Entry, entries: &[Entry]) -> View {
    let id = name_to_id(&entry.name);
    view! {
        article [id=id, class="item"] {
            hr/

            h1 [class="name"] {
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

            [if let Some(subtitle) = &entry.subtitle { view! {
                h4 [class="subtitle"] { [subtitle] }
            }} else { view!{} }]

            p [class="description"] { [&entry.description] }

            div [class="sources"] {
                [:if (!entry.sources.is_empty()) {
                    p { i { "Sources:" } }
                    ul { [:for source in &entry.sources {
                        li { @short_link [source, None] }
                    }] }
                } else {
                    p { b { "No sources!" } }
                }]
            }

            [if let Some(review) = &entry.review { view!{
                p [class="review"] {
                    i { "Critic's Review:" }
                    ~ span {
                        [review]
                    }
                }
            } } else { view!{} }]

            div [class="related"] {
                details {
                    summary { "Related" }
                    ul { [:for other in entries {
                        [:if (
                            other.name != entry.name
                            && do_lists_intersect(&other.tags, &entry.tags)
                        ) {
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

fn short_link(link: &str, text: Option<&str>) -> View {
    // long version is for print
    view! {
        span [class="short-link"] {
            [if let Some(text) = text { view! {
                span [class="short"] {
                    a [href=link] { [text] }
                }
                span [class="long", style="display:none"] {
                    span [class="text"] { [text] }
                    ":" ~
                    a [href=link] { [link] }
                }
            }} else { view! {
                span [class="short"] {
                    a [href=link] { [shorten_url(link).unwrap_or(link)] }
                }
                span [class="long", style="display:none"] {
                    a [href=link] { [link] }
                }
            }}]
        }
    }
}

fn at_404() -> Document {
    view! {
        @use_base
        div [class="not-found"] {
            h2 {
                "... a 404 Error? -"
                ~ i { "NO!" }
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
        }

        h1 [class="heading"] {
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
    for a in a {
        if b.contains(a) {
            return true;
        }
    }
    false
}

fn shorten_url(link: &str) -> Option<&str> {
    let mut split_protocol = link.split("://");
    split_protocol.next();
    let full_domain = split_protocol.next()?.split("/").next()?;
    if !full_domain.starts_with("www.") {
        return Some(full_domain);
    }
    let Some(index) = full_domain.find('.') else {
        return Some(full_domain);
    };
    let (_, domain) = full_domain.split_at(index + 1);
    if domain.is_empty() {
        return Some(full_domain);
    }
    Some(domain)
}

