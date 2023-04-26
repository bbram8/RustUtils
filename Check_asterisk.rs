pub(crate) fn main()
{
    let mut a = 10;
    let mut p = &a;

    println!("p is: {}", p);
    println!("p's value is {}", *p);
    println!("Address of p is {}", p);
    //a = a + 10;
    p = &20;
    println!("p's value is {}", *p);
    println!("Address of p is {}", p);


}
