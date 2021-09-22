pub fn count_words(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let mut wordcount = 0;
    for line in lines {
        for _word in line.split_whitespace() {
            wordcount += 1;
        }
    }
    wordcount
}
