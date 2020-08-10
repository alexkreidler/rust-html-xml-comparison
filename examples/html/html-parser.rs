use html_parser::Dom;

fn main() {
    let html = r#"
        <!doctype html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>Html parser</title>
            </head>
            <body>
                <h1 id="a" class="b c">Hello world</h1>
                </h1> <!-- comments & dangling elements are ignored -->
            </body>
        </html>"#;

    assert!(Dom::parse(html).is_ok());
}
