use std::collections::HashMap;

/// Create an array that will contain all the letters of word and her index from 1 to word.len()-1
/// # Arguments
///
/// * `word` - A slice of string containing the letters to put in the HashMap
///
/// # Return
/// 
/// * <HashMap<&'static str, usize>
/// 
fn bad_char_table(word: &'static str) -> HashMap<char, usize> {
    let mut table = HashMap::new();
    let mut index = word.len()-1;

    for lettre in word.chars() {
        if index>0 {
            table.insert(lettre, index);
            index-= 1 ;
        }
    }
    return table;
}


/// Return the index of the first occurrence of `needle` in `haystack` with Boyer Moore Horspool algorithm
/// # Arguments
///
/// * `needle` - A slice of string, that's the word we're looking for
/// * `haystack` - The place where we go to look for `needle`
///
/// # Examples
/// 
/// ```
/// 
/// use rust_algo::searching::boyer_moore_horspool::boyer_moore_horspool;
/// 
/// fn main() {
///     let worlds = String::from("Hello John, how are you?");
///     let word = "are";
///    println!("{}", boyer_moore_horspool(word, worlds));
/// }
/// 
/// ```
/// 
///  # Returns 
/// 
/// * `i64` - the index of the first occurence of `needle` or -1 if not found
/// 
pub fn boyer_moore_horspool(needle : &'static str, haystack: String ) -> i64 {
    //make the index table of the letters of "needle"
    let table = bad_char_table(needle);
    
    let mut skip :usize  = 0; // main position
    let mut i : usize;
    
    while haystack.len() - skip >= needle.len()
    {
        i = needle.len() -1; //beacause we read backwards
        while haystack.as_bytes()[skip+i] as char == needle.as_bytes()[i] as char //if it matches
        {
            if i == 0 //if we arrive to head of word, we found first occurence of needle
            {
                return skip as i64;
            }
            i-=1;
        }
        //we go next position (skip + length of needle -1) and we look if we the character matches a character from our table
        skip +=  match table.get(&(haystack.as_bytes()[skip+needle.len() -1] as char)) {
            Some(x) =>  *x,
            None => needle.len(), //
        };
    }
    
    //if not found
    return -1;
}

#[cfg(test)]
mod boyer_moore_horspool_tests {
    use super::*;
    #[test]
    fn good() {
        assert_eq!(boyer_moore_horspool("dont", String::from("I dont know")), 2);
        assert_eq!(boyer_moore_horspool("I", String::from("I dont know")), 0);
        assert_eq!(boyer_moore_horspool("know", String::from("I dont know")), 7);
        assert_eq!(boyer_moore_horspool("fish", String::from("I dont know")), -1);
        assert_eq!(boyer_moore_horspool("i", String::from("I dont know")), -1);
    }
    #[test]
    fn notgood() {
    }

}