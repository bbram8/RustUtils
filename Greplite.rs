pub(crate) fn main(){

    let search_term = "pages";
    let quote = "\
    Every face, every shop, bedroom window, public house and dark square is a picture. \
    feverishly turned in search of what? It is the same with books. \
    What do we seek in millions of pages?";
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
