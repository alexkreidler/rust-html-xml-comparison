use std::fs::File;
use xmltree::Element;

fn main() -> Result<(), anyhow::Error> {
    let data: &'static str = r##"
<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<names>
    <name first="bob" last="jones" />
    <name first="elizabeth" last="smith" />
</names>
"##;

    let mut names_element = Element::parse(data.as_bytes()).unwrap();

    println!("{:#?}", names_element);
    {
        // get first `name` element
        let name = names_element
            .get_mut_child("name")
            .expect("Can't find name element");
        name.attributes.insert("suffix".to_owned(), "mr".to_owned());
    }
    names_element.write(std::io::stdout())?;
    Ok(())
}
