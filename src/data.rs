/// Some basic, fully compliant XML
pub const basic: &'static str = r#"<tag1 att1 = "test">
<tag2><!--Test comment-->Test</tag2>
<tag2>
    Test 2
</tag2>
</tag1>"#;

pub const basic2: &'static str = r##"
<?xml version="1.0" encoding="utf-8" ?>
<names>
    <name first="bob" last="jones" />
    <name first="elizabeth" last="smith" />
</names>
"##;

/// Invalid XML, but code that would be automatically fixed by an HTML parser
pub const invalid: &'static str = r#"
<outer>
    <invalid>
    <nest attr="hi">
    <br />
    </invalid>
    </nest order="example">
</outer>
"#;

/// More invalid, unclosed, autofixed
pub const invalid2: &'static str = r#"
<unclosed>
<h1>Test</h1>
"#;

pub const invalid3: &'static str = r#"
<unclosed>
<h1>Test</h2>
"#;

/// self closed
pub const self_closed: &'static str = r#"
<page>
    <p>Hello</p>
    <br/>
    <img src="google.com" />
</page>
"#;

pub const html: &'static str = r#"
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
