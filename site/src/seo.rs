pub fn set_page_meta(title: &str, description: &str) {
    set_page_meta_with_noindex(title, description, false)
}

pub fn set_page_meta_with_noindex(title: &str, description: &str, noindex: bool) {
    let window = web_sys::window().expect("no window");
    let doc = window.document().expect("no document");
    doc.set_title(title);

    let update_attr = |selector: &str, attr: &str, value: &str| {
        if let Ok(Some(el)) = doc.query_selector(selector) {
            let _ = el.set_attribute(attr, value);
        }
    };

    update_attr("meta[name='description']", "content", description);
    update_attr("meta[property='og:title']", "content", title);
    update_attr("meta[property='og:description']", "content", description);
    update_attr("meta[property='og:url']", "content", "");
    update_attr("meta[property='og:image']", "content", "");
    update_attr("meta[property='og:site_name']", "content", "");

    if noindex {
        update_attr("meta[name='robots']", "content", "noindex");
    } else if let Ok(Some(el)) = doc.query_selector("meta[name='robots']") {
        let _ = el.remove();
    }
}
