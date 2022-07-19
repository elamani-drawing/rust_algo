use std::cmp;

/// Make distance of levenstein between `word` and `word2`.
/// # Arguments
///
/// * `word` - A character string
/// * `word2` - A character string 
///
/// # Examples
/// 
/// ```
/// use rust_algo::math::distance_from_levenshtein::distance_from_levenshtein;
/// fn main() {
///     println!("{}", distance_from_levenshtein("niche", "chien"));
/// }
/// ```
/// 
///  # Returns 
/// 
/// * `u64` - A measurement of the difference between the 2 strings
/// 
pub fn distance_from_levenshtein(word : &'static str, word2 : &'static str ) -> u64 {
    let mut distance :Vec<Vec<u64>> = Vec::new(); // cross table 
    //initialisation of first row of table
    distance.push(Vec::new());
    for i in 0..=word.len() {
        distance[0].push(i as u64);
    }
    //initialisation of first column of table
    for j in 1..=word2.len() {
        distance.push(Vec::new());
        distance[j].push(j as u64);
    }

    let mut top : u64 ;
    let mut left : u64 ;
    let mut top_left : u64 ;
    let mut cost : u64;
    //we loop through the array to dynamically calculate the values ​​of each cell
    for i in 1..=word.len() {
        for j in 1..=word2.len(){
            //cost calculation: if the 2 letters are different 1 otherwise 0
            if word.as_bytes()[i-1] as char == word2.as_bytes()[j-1] as char{
                cost = 0
            }else{
                cost = 1
            }

            //find minimum of different distance [ (tab[i-1][j]+1 && tab[i][j-1] +1) && tab[i-1][j-1]+cost ] and calculate value of current cell
            top = distance[i-1][j]+1;
            left = distance[i][j-1] +1;
            top_left =distance[i-1][j-1] + cost;

            distance[i].push(cmp::min(
                cmp::min( top
                     , left
                ), top_left
                
            ))
        }
    }
    
    //the levenshtein distance will be the last cell
    return distance[word.len()][word2.len()];
}

#[cfg(test)]
mod distance_from_levenshtein_tests {
    use super::*;
    #[test]
    fn good() { 
        let words: [[&str; 2]; 2] = [
            ["niche", "chien"],
            ["maison", "maçon"]
        ];
        let solutions : [u64;2] = [4,2];
        for i in 0..words.len() {
            assert_eq!(distance_from_levenshtein(words[i][0],words[i][1] ), solutions[i]);
        }
    }
}