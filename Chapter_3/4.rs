
//This function takes an int as an input and returns a Result enum as an output..
fn guess(num:i32)->Result< i32,&'static str>
{
    if(num==42)
    {
        Ok(42)
    }
    else
    {
        Err("OOPS..You did not enter 42")
    }
}

fn main()
{
    // Here we receive 2 enums n1 and n2. In these enums, we receive ok or err
    // We unwrap then enums using match condition.
    // we can also use .unwrap() but that would cause an error if we try to display the value of n1 and n2
    // as it will be considered as an error
     let n1= guess(42);
     let n2= guess(41);
    //using match, thread wont panic and stop, even when an enum Err is recieved.
     match n1{
         Ok(xx) => {println!("The first result is -> {}",xx );},
         Err(xx) => {println!("{}",xx);}
     }
     
     match n2{
        Ok(xx) => {println!("The first result is -> {}",xx );},
        Err(xx) => {println!("{}",xx);}
    }
}