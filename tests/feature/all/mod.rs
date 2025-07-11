use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/all/schema.xsd",
        "tests/feature/all/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/all/schema.xsd",
        "tests/feature/all/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/all/schema.xsd",
        "tests/feature/all/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/all/schema.xsd",
        "tests/feature/all/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/all/example/default.xml");

    assert_eq!(obj.once, 222);
    assert_eq!(obj.optional, None);
    assert_eq!(obj.once_specify, 444);
    assert_eq!(obj.twice_or_more, &[111, 333, 555]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::Foo;

    let obj = Foo {
        once: 222,
        optional: None,
        once_specify: 444,
        twice_or_more: vec![111, 333, 555],
    };

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/feature/all/example/serialize.xml");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>("tests/feature/all/example/serde.xml");

    assert_eq!(obj.once, 111);
    assert_eq!(obj.optional, Some(222));
    assert_eq!(obj.once_specify, 333);
    assert_eq!(obj.twice_or_more, &[444, 555, 666]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Foo;

    let obj =
        crate::utils::serde_quick_xml_read_test::<Foo, _>("tests/feature/all/example/serde.xml");

    assert_eq!(obj.once, 111);
    assert_eq!(obj.optional, Some(222));
    assert_eq!(obj.once_specify, 333);
    assert_eq!(obj.twice_or_more, &[444, 555, 666]);
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
