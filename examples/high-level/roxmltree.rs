use anyhow::Result;

fn main() -> Result<()> {
    // Find element by id.
    let doc = roxmltree::Document::parse("<rect id='rect1'/>").unwrap();
    let elem = doc
        .descendants()
        .find(|n| n.attribute("id") == Some("rect1"))
        .unwrap();
    assert!(elem.has_tag_name("rect"));
    Ok(())
}
