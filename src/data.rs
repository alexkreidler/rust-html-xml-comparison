/// Some basic, fully compliant XML
pub const basic: &'static str = r#"<tag1 att1 = "test">
<tag2><!--Test comment-->Test</tag2>
<tag2>
    Test 2
</tag2>
</tag1>"#;

pub const basic2: &'static str = r##"
<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<names>
    <name first="bob" last="jones" />
    <name first="elizabeth" last="smith" />
</names>
"##;

/// Invalid XML, but code that would be automatically fixed by an HTML parser
pub const invalid: &'static str = r#"
<outer>
    <invalid>
    <nest>
    </invalid>
    </nest order="example">
</outer>
"#;

/// More invalid, unclosed, autofixed
pub const invalid2: &'static str = r#"
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
