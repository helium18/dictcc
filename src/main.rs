use std::env;

mod dictcc;

#[allow(clippy::main_recursion)]
fn main() {
    let word = env::args().collect::<Vec<String>>();
    let word = word[1].as_str();

    let word = word
        .replace("^ae", "ä")
        .replace("^Ae", "Ä")
        .replace("^ue", "ü")
        .replace("^Ue", "Ü")
        .replace("^oe", "ö")
        .replace("^Oe", "Ö")
        .replace("!^ss", "ß");

    let word = word.as_str();

    let definitions = dictcc::dictcc::translate("", "", word);
    if let Err(err) = definitions {
        eprintln!("Something went wrong 🤔.\n Error: {}", err);
        main();
    } else {
        definitions
            .unwrap()
            .into_iter()
            .enumerate()
            .for_each(|(index, defn)| {
                println!("{}. {}", index + 1, defn);
            })
    }
}
