/*
  -> show_str() function takes input a string and display a line above and below that string

*/ 
fn show_str(msg:&str)
{
    // the line below is a way of creating strings og same chars.
    let s1 = std::iter::repeat('-').take(msg.len() +2 ).collect::<String>();
    let s2 = std::iter::repeat('-').take(msg.len() +2).collect::<String>();
    println!(" {} \n {} \n {}",s1,&msg,s2 );
}

fn main()
{
    let s="HI..".to_string();
    show_str(&s);

}