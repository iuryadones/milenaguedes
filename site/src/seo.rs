pub fn set_page_meta(title: &str, description: &str) {
    let doc = web_sys::window().unwrap().document().unwrap();
    doc.set_title(title);

    let update_attr = |selector: &str, attr: &str, value: &str| {
        if let Some(el) = doc.query_selector(selector).unwrap() {
            let _ = el.set_attribute(attr, value);
        }
    };

    update_attr("meta[name='description']", "content", description);
    update_attr("meta[property='og:title']", "content", title);
    update_attr("meta[property='og:description']", "content", description);
}
