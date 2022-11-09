mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};
use sycamore::prelude::view;

#[perseus::main(perseus_warp::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .index_view(|cx| {
            view! { cx,
                html {
                    head {
                        meta(charset = "UTF-8")
                        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
                        link(rel = "stylesheet", href = ".perseus/static/styles.css")
                    }
                    body {
                        PerseusRoot()
                    }
                }
            }
        })
}
