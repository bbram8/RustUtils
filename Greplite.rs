pub(crate) fn main(){

    let search_term = "pages";
    let quote = "\
    In every soul there is greatness and in every book there is knowledge. \
    What do we seek in millions of pages then?";
    // quote is a good string object. Rust provides a lot of string functionality
    let mut line_num : usize = 1;
    for line in quote.lines(){
        //println!("{}",line);
        if line.contains(search_term){
            println!("{}:{}", line_num, line);
        }
        line_num += 1;
    }


}
