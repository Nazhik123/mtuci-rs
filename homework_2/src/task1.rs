/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    find_term(SEARCH_TERM, QUOTE);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let a = 'z'.to_string();
    let mut n: i32 = 0;
    for line in QUOTE.lines() {
        if line.contains(SEARCH_TERM) {
            let a = line;
            n += 1;
            let a = format!("{}:{}",a,n);
            break;
        } else {
            
        }
    }
    a
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
