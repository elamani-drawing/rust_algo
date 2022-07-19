use rust_algo::searching::boyer_moore_horspool::boyer_moore_horspool;

fn main() {
    let worlds = String::from("Hello John, how are you?");
    let word = " ls";
    println!("{}", boyer_moore_horspool(word, worlds));
}
 