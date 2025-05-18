use plist_plus::{array, dict, Uid, Value};
use std::time::Duration;

#[test]
fn ascii_animals() {
    let plist = plist_plus::from_file("./tests/ascii-animals.plist").unwrap();
    let dict: Value = dict!(
        "AnimalColors" => dict!(
            "lamb" => "black",
            "pig" => "pink",
            "worm" => "pink"
        ),
        "AnimalSmells" => dict!(
            "lamb" => "lambish",
            "pig" => "piggish",
            "worm" => "wormy"
        ),
        "AnimalSounds" => dict!(
            "Lisa" => "Why is the worm talking like a lamb?",
            "lamb" => "baa",
            "pig" => "oink",
            "worm" => "baa"
        )
    )
    .into();
    assert!(dict == plist)
}

#[test]
fn ascii_sample() {
    let plist = plist_plus::from_file("./tests/ascii-sample.plist").unwrap();
    println!("{plist:?}");
    let dict: Value = dict!(
        "KeyName1" => "Value1",
        "AnotherKeyName" => "Value2",
        "Something" => array!("ArrayItem1", "ArrayItem2", "ArrayItem3"),
        "Key4" => "0.10",
        "KeyFive" => dict!(
            "Dictionary2Key1" => "Something",
            "AnotherKey" => "Somethingelse"
        )
    )
    .into();
    assert!(dict == plist)
}

#[test]
#[should_panic]
fn binary_circular_array() {
    plist_plus::from_file("./tests/binary_circular_array.plist").unwrap();
}

#[allow(non_snake_case)]
#[test]
fn binary_NSKeyedArchiver() {
    let plist = plist_plus::from_file("./tests/binary_NSKeyedArchiver.plist").unwrap();

    let data: Vec<u8> = vec![
        3, 2, 8, 1, 12, 6, 21, 1, 23, 1, 25, 1, 27, 2, 31, 8, 44, 1, 48, 1, 51, 1, 53, 3, 60, 2,
        64, 2, 71, 1, 76, 1, 83, 1, 86, 2, 91, 1, 100, 2, 111, 2, 120, 2, 124, 1, 141, 1, 1, 144,
        1, 1, 155, 1, 1, 160, 1, 2, 166, 1, 2, 221, 1, 1, 188, 2, 1, 128, 8, 3, 132, 8, 1, 134, 8,
        5, 147, 8, 1, 152, 8, 3, 156, 8, 1, 159, 8, 1, 166, 8, 1, 176, 8, 1, 188, 8, 1, 199, 8, 1,
        128, 64, 1,
    ];

    let dict: Value = dict!(
        "$version" => 100000,
        "$objects" => array!(
            "$null",
            dict!(
                "NSRangeCount" => 42,
                "$class" => Uid::new(4),
                "NSRangeData" => Uid::new(2)
            ),
            dict!(
                "NS.data" => data,
                "$class" => Uid::new(3)
            ),
            dict!(
                "$classname" => "NSMutableData",
                "$classes" => array!("NSMutableData", "NSData", "NSObject")
            ),
            dict!(
                "$classname" => "NSMutableIndexSet",
                "$classes" => array!("NSMutableIndexSet", "NSIndexSet", "NSObject")
            )
        ),
        "$archiver" => "NSKeyedArchiver",
        "$top" => dict!("foundItems" => Uid::new(1))
    )
    .into();
    assert!(dict == plist)
}

#[test]
#[should_panic]
fn binary_zero_offset_size() {
    let plist = plist_plus::from_file("./tests/binary_zero_offset_size.plist").unwrap();
    println!("{plist:?}");
}

#[test]
fn binary() {
    let plist = plist_plus::from_file("./tests/binary.plist").unwrap();

    let data: Vec<u8> = vec![0, 0, 0, 190, 0, 0, 0, 3, 0, 0, 0, 30, 0, 0, 0];
    let dict: Value = dict!(
        "Author" => "William Shakespeare",
        "Birthdate" => Duration::from_secs(358860726),
        "EmptyArray" => array!(),
        "IsNotFalse" => false,
        "SmallestNumber" => i64::MIN,
        "EmptyDictionary" => dict!(),
        "Height" => 1.6,
        "Lines" => array!(
            "It is a tale told by an idiot,     ",
            "Full of sound and fury, signifying nothing."
        ),
        "Death" => 1564,
        "Blank" => "",
        "BiggestNumber" => u64::MAX,
        "IsTrue" => true,
        "Data" => data
    )
    .into();
    assert!(dict == plist)
}

#[test]
fn book() {
    let plist = plist_plus::from_file("./tests/book.plist").unwrap();
    let dict: Value = dict!(
        "Title" => "Great Expectations",
        "Author" => "Charles Dickens",
        "Excerpt" => "Whether I should have made out this object so soon, if there had been no fine lady sitting at it, I cannot say. In an armchair, with an elbow resting on the table and her head leaning on that hand, sat the strangest lady I have ever seen, or shall ever see.",
        "CopiesSold" => 123456789
    ).into();
    assert!(dict == plist)
}

#[test]
fn utf16_bplist() {
    let plist = plist_plus::from_file("./tests/utf16_bplist.plist").unwrap();
    let dict: Value = dict!(
        "name" => "★ or better",
        "longText" => "The sun was shining on the sea, \nShining with all his might: \nHe did his very best to make \nThe billows smooth and bright\n-- And this was odd, because it was \nThe middle of the night. \n\nThe moon was shining sulkily, \nBecause she thought the sun\nHad got no business to be there \nAfter the day was done\n-- \"It's very rude of him,\" she said, \n\"To come and spoil the fun!\" \n\nThe sea was wet as wet could be,\nThe sands were dry as dry. \nYou could not see a cloud, because \nNo cloud was in the sky: \nNo birds were flying overhead\n-- There were no birds to fly. \n\nIn a Wonderland they lie \nDreaming as the days go by, \nDreaming as the summer die.\n★"
    ).into();
    assert!(dict == plist)
}

#[test]
#[should_panic]
fn xml_error() {
    plist_plus::from_file("./tests/xml_error.plist").unwrap();
}

#[test]
fn xml_animals() {
    let plist = plist_plus::from_file("./tests/xml-animals.plist").unwrap();
    let dict: Value = dict!(
        "AnimalColors" => dict!(
            "lamb" => "black",
            "pig" => "pink",
            "worm" => "pink"
        ),
        "AnimalSmells" => dict!(
            "lamb" => "lambish",
            "pig" => "piggish",
            "worm" => "wormy"
        ),
        "AnimalSounds" => dict!(
            "Lisa" => "Why is the worm talking like a lamb?",
            "lamb" => "baa",
            "pig" => "oink",
            "worm" => "baa"
        )
    )
    .into();
    assert!(dict == plist)
}

#[test]
fn xml() {
    let plist = plist_plus::from_file("./tests/xml.plist").unwrap();
    let data: Vec<u8> = vec![0, 0, 0, 190, 0, 0, 0, 3, 0, 0, 0, 30, 0, 0, 0];
    let dict: Value = dict!(
        "Author" => "William Shakespeare",
        "Lines" => array!(
            "It is a tale told by an idiot,     ",
            "Full of sound and fury, signifying nothing."
        ),
        "Death" => 1564,
        "Height" => 1.6,
        "Data" => data,
        "Birthdate" => Duration::from_secs(358860726),
        "Blank" => "",
        "BiggestNumber" => u64::MAX,
        "SmallestNumber" => i64::MIN,
        "HexademicalNumber" => 0xDEADBEEF as i64,
        "IsTrue" => true,
        "IsNotFalse" => false
    )
    .into();
    assert!(dict == plist)
}
