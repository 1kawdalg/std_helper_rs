#[macro_use] extern crate std_helper;
use std_helper::StrHelper;

use std::io::Result;

#[test]
fn init_helper() -> Result<()> {
    let helper = StrHelper::new("Language Crab is Rust... Without changes... Sh...");
    assert_eq!(helper.as_str(), "Language Crab is Rust... Without changes... Sh...");

    Ok(())
}

#[test]
fn macro_test() -> Result<()> {
    let empty_helper = str!();
    assert_eq!(empty_helper, StrHelper::empty());

    let once_helper = str!("Hello!");
    assert_eq!(once_helper, StrHelper::new("Hello!"));

    let helper = str!("Hi, Python!", " ", "How do you ", "do?");
    assert_eq!(helper, StrHelper::new("Hi, Python! How do you do?"));

    Ok(())
}

#[test]
fn split_test() -> Result<()> {
    let helper = str!("hello");

    let l = helper.split_on_the_sides(("e", "lo")).unwrap();
    assert_eq!(l.as_str(), "l");

    let lo = helper.split_and_get_last_string("el").unwrap();
    assert_eq!(lo, "lo");

    let lol = lo + l.as_str();
    assert_eq!(lol, "lol");

    Ok(())
}

#[test]
fn split_at_test() -> Result<()> {
    let helper = str!("first partsecond part");

    let (first, second) = helper.split_at(10);
    assert_eq!(first.as_ref().unwrap().as_str(), "first part");
    assert_eq!(second.as_ref().unwrap().as_str(), "second part");

    let space = first
        .as_ref()
        .unwrap()
        .split_str_on_the_sides_at((5, 6))
        .unwrap();

    assert_eq!(space, " ");

    Ok(())
}

#[test]
fn base_fns_test() -> Result<()> {
    let chars = [
        '!', '@', '#', '$', '%',
        '^', '&', '*', '(', ')',
        '-', '_', '=', '+', '?']
        .as_ref();

    let helper = StrHelper::from_chars(chars);

    assert_eq!(helper.as_str(), "!@#$%^&*()-_=+?");
    assert_eq!(helper.to_string(), "!@#$%^&*()-_=+?");

    assert_eq!(helper.contains("!"), true);
    assert_eq!(helper.contains("rust"), false);

    assert_eq!(helper.as_chars(), chars.to_vec());
    assert_eq!(StrHelper::other_as_chars("hello"), ['h', 'e', 'l', 'l', 'o'].as_ref());

    Ok(())
}

#[test]
fn insert_test() -> Result<()> {
    let mut helper = str!("Ni");

    helper.push("hao :>");
    assert_eq!(helper.as_str(), "Nihao :>");

    helper.insert_at_the_beginning("Oh... ");
    assert_eq!(helper.as_str(), "Oh... Nihao :>");

    helper.insert(", Rusty Crabs", 2);
    assert_eq!(helper.as_str(), "Oh, Rusty Crabs... Nihao :>");

    Ok(())
}

#[test]
fn remove_test() -> Result<()> {
    let mut helper = str!("Hello, @@@F****** ***T *I***, World!");

    assert_eq!(
        helper.remove("F****** ***T *I***, ").unwrap(),
        helper.to_string(),
        "Hello, @@@World!"
    );

    assert_eq!(
        helper.remove_at((7, 10)).unwrap(),
        "Hello, World!"
    );

    Ok(())
}

#[test]
fn move_test() -> Result<()> {
    let mut helper = str!("!e, lloWorldH");

    assert_eq!(
        helper.move_to((4, 7), 2).unwrap(),
        "!ello, WorldH"
    );

    assert_eq!(helper.switch_beginning_and_end().unwrap(), "Hello, World!");

    Ok(())
}