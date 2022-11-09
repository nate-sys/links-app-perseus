use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        main {
            div(class="container-hero") {
                img(src="https://picsum.photos/200", alt="avatar")
                h3 { "Some Person" }
                p {
                    " Ipsum voluptatum nam ipsam veritatis laudantium Dicta officiis consectetur impedit quasi tempora quaerat? Neque neque rerum consequatur sint commodi Qui quaerat minus saepe repellendus quia obcaecati molestias Rem nisi aliquam?  "
                }
            }
            div(class="container-links") {
                a(href="#"){ "link1" }
                a(href="#"){ "link2" }
                a(href="#"){ "link3" }
                a(href="#"){ "link4" }
                a(href="#"){ "link5" }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx, title { "Home" } }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
