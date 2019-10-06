use trigram::find_words_iter;

fn main() {
    let haystack =
        "Did you know that bufalo buffalow Bungalo biffalo buffaloo huffalo snuffalo fluffalo?";
    let needle = "buffalo";
    for m in find_words_iter(needle, haystack, 0.3) {
        println!("{}", m.as_str());
    }
}
