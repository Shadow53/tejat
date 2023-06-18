use pretty_assertions::assert_eq;
use tejat::{
    blockquote,
    h1,
    h2,
    h3,
    link,
    list_item,
    preformatted,
    text,
    gemtext::{parse_lines, RawLine},
};

#[test]
fn test_parse_empty_document() {
    let out = parse_lines("").unwrap();
    assert_eq!(out, vec![]);
}

#[test]
fn test_parse_big_document() {
    let out = parse_lines(BIG_DOCUMENT).unwrap();
    let expected = get_parsed_big_document();
    assert_eq!(out, expected);
}


fn get_parsed_big_document() -> Vec<RawLine<'static>> {
    vec![
        h1!("Test Document"),
        text!(""),
        link!("https://example.com" -> "An example HTTPS link"),
        link!("gemini://example.com/test/?query=something&test" -> "An example Gemini link"),
        link!("gemini://example.com/bare-link"),
        text!(""),
        text!("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer turpis diam, iaculis at est non, euismod sodales elit. Duis vitae fermentum neque, sodales vulputate eros. Aenean lobortis ante sit amet sapien fermentum varius. Fusce dictum nulla eget dignissim mollis. Ut tristique urna pellentesque est iaculis pharetra. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam erat volutpat. Duis nisl augue, ullamcorper viverra metus quis, sagittis fermentum nisl. Maecenas mi lorem, blandit non vestibulum at, eleifend sit amet nisi. Mauris non ornare tortor. Curabitur rutrum malesuada quam."),
        text!(""),
        h2!("Heading 2"),
        text!(""),
        text!("Vestibulum pulvinar consequat tellus, ac faucibus arcu iaculis id. In nec convallis leo. Nullam vitae purus feugiat, maximus felis in, ultrices risus. Sed vel arcu non nunc tincidunt cursus. Nam sit amet metus sapien. Integer maximus ante sit amet arcu bibendum porttitor. Duis tristique ullamcorper mi, vel blandit urna volutpat a. Nulla in pretium velit. Curabitur tempor metus elit, sed dignissim neque ornare eu. Morbi non magna a ex aliquet posuere. Phasellus pellentesque placerat eleifend."),
        text!(""),
        blockquote!("Block quote line 1"),
        blockquote!("Block quote line 2"),
        text!(""),
        link!("/root-relative/link"),
        link!("relative/link"),
        text!(""),
        text!("Nulla facilisi. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vivamus hendrerit mauris eu mauris pretium dignissim. Ut a ligula venenatis, viverra lacus sed, rhoncus erat. Donec posuere diam nunc, a auctor leo elementum non. Nunc ornare turpis id purus tempus vulputate. Pellentesque ullamcorper turpis arcu, et auctor lectus egestas vel. Sed diam ante, maximus non sem non, sollicitudin consectetur dui. Vivamus dolor dolor, blandit vitae enim sit amet, hendrerit facilisis ligula. Nam porta quis mi eu maximus. Aenean nisi quam, sollicitudin nec pulvinar non, consectetur eget mi. Nulla porta feugiat nibh sed viverra. Donec luctus augue sit amet metus dictum, quis elementum augue cursus."),
        text!(""),
        text!(""),
        preformatted!("banner": r#"o.oOOOo.
 o     o
 O     O
 oOooOO.
 o     `O .oOoO' 'OoOo. 'OoOo. .oOo. `OoOo.
 O      o O   o   o   O  o   O OooO'  o
 o     .O o   O   O   o  O   o O      O
 `OooOO'  `OoO'o  o   O  o   O `OoO'  o"#),
        text!(""),
        text!(""),
        h3!("Heading 3"),
        text!(""),
        text!("Curabitur mauris purus, rutrum in tellus nec, rhoncus volutpat ipsum. Maecenas tempus dui eget sapien lobortis, eget scelerisque massa placerat. Maecenas id mattis augue. Phasellus ac posuere elit. Praesent eget arcu sodales libero porta sollicitudin quis ultrices lorem. Morbi ullamcorper sollicitudin dui, a pellentesque ante ultrices vel. Aenean pretium felis orci, vel efficitur turpis hendrerit sit amet. Ut vel dui tincidunt, vestibulum diam ut, elementum eros. Cras auctor eu sem tincidunt maximus. Maecenas commodo augue eros, finibus faucibus metus condimentum eu. Sed interdum ipsum nec neque congue, et egestas sapien egestas."),
        text!(""),
        text!("Vestibulum malesuada id lacus rutrum luctus. Proin sit amet feugiat leo. Curabitur aliquet, dui eget scelerisque auctor, orci ante accumsan neque, quis semper est massa at lectus. In vulputate felis at turpis dapibus vulputate. Donec interdum eros sit amet risus maximus lacinia. Pellentesque consequat, risus quis vulputate mollis, elit quam laoreet erat, a sagittis orci enim venenatis felis. Nunc consequat vulputate magna, at finibus nibh facilisis et. Ut sollicitudin blandit vulputate. Sed at mi hendrerit, pretium nunc at, feugiat felis. Sed varius elementum feugiat. Integer tempus maximus imperdiet. Ut sed dolor vitae erat fringilla hendrerit vitae quis orci."),
        text!(""),
        list_item!("List item 1"),
        list_item!("List item 2"),
        text!(""),
        list_item!("List item 3"),
        text!(""),
        text!(""),
    ]
}

const BIG_DOCUMENT: &str = r#"
# Test document

=> https://example.com  An example HTTPS link
=>gemini://example.com/test/?query=something&test An example Gemini link
=>        gemini://example.com/bare-link

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer turpis diam, iaculis at est non, euismod sodales elit. Duis vitae fermentum neque, sodales vulputate eros. Aenean lobortis ante sit amet sapien fermentum varius. Fusce dictum nulla eget dignissim mollis. Ut tristique urna pellentesque est iaculis pharetra. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam erat volutpat. Duis nisl augue, ullamcorper viverra metus quis, sagittis fermentum nisl. Maecenas mi lorem, blandit non vestibulum at, eleifend sit amet nisi. Mauris non ornare tortor. Curabitur rutrum malesuada quam.

## Heading 2

Vestibulum pulvinar consequat tellus, ac faucibus arcu iaculis id. In nec convallis leo. Nullam vitae purus feugiat, maximus felis in, ultrices risus. Sed vel arcu non nunc tincidunt cursus. Nam sit amet metus sapien. Integer maximus ante sit amet arcu bibendum porttitor. Duis tristique ullamcorper mi, vel blandit urna volutpat a. Nulla in pretium velit. Curabitur tempor metus elit, sed dignissim neque ornare eu. Morbi non magna a ex aliquet posuere. Phasellus pellentesque placerat eleifend.

> Block quote line 1
> Block quote line 2

=>/root-relative/link
=>    relative/link

Nulla facilisi. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Vivamus hendrerit mauris eu mauris pretium dignissim. Ut a ligula venenatis, viverra lacus sed, rhoncus erat. Donec posuere diam nunc, a auctor leo elementum non. Nunc ornare turpis id purus tempus vulputate. Pellentesque ullamcorper turpis arcu, et auctor lectus egestas vel. Sed diam ante, maximus non sem non, sollicitudin consectetur dui. Vivamus dolor dolor, blandit vitae enim sit amet, hendrerit facilisis ligula. Nam porta quis mi eu maximus. Aenean nisi quam, sollicitudin nec pulvinar non, consectetur eget mi. Nulla porta feugiat nibh sed viverra. Donec luctus augue sit amet metus dictum, quis elementum augue cursus.


```banner
o.oOOOo.
 o     o
 O     O
 oOooOO.
 o     `O .oOoO' 'OoOo. 'OoOo. .oOo. `OoOo.
 O      o O   o   o   O  o   O OooO'  o
 o     .O o   O   O   o  O   o O      O
 `OooOO'  `OoO'o  o   O  o   O `OoO'  o
```


###Heading 3

Curabitur mauris purus, rutrum in tellus nec, rhoncus volutpat ipsum. Maecenas tempus dui eget sapien lobortis, eget scelerisque massa placerat. Maecenas id mattis augue. Phasellus ac posuere elit. Praesent eget arcu sodales libero porta sollicitudin quis ultrices lorem. Morbi ullamcorper sollicitudin dui, a pellentesque ante ultrices vel. Aenean pretium felis orci, vel efficitur turpis hendrerit sit amet. Ut vel dui tincidunt, vestibulum diam ut, elementum eros. Cras auctor eu sem tincidunt maximus. Maecenas commodo augue eros, finibus faucibus metus condimentum eu. Sed interdum ipsum nec neque congue, et egestas sapien egestas.

Vestibulum malesuada id lacus rutrum luctus. Proin sit amet feugiat leo. Curabitur aliquet, dui eget scelerisque auctor, orci ante accumsan neque, quis semper est massa at lectus. In vulputate felis at turpis dapibus vulputate. Donec interdum eros sit amet risus maximus lacinia. Pellentesque consequat, risus quis vulputate mollis, elit quam laoreet erat, a sagittis orci enim venenatis felis. Nunc consequat vulputate magna, at finibus nibh facilisis et. Ut sollicitudin blandit vulputate. Sed at mi hendrerit, pretium nunc at, feugiat felis. Sed varius elementum feugiat. Integer tempus maximus imperdiet. Ut sed dolor vitae erat fringilla hendrerit vitae quis orci.

* List item 1
* List item 2

* List item 3

"#;
