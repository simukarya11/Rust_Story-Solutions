/* 
  -> Display function takes a vector as an input nd display its elements
*/

fn display(vector:Vec<i32>)
{
    let size=vector.len();

    // Below is a syntax through which you can iterate over a vector by getting its index and value both
    for (index,num) in vector.iter().enumerate()
    {
        if index == size-1
        {
            print!("{}\n ",num );
        }
        else
        {
            print!("{}, ",num );
        }
        
    }

}

fn main()
{
    let vector:Vec<i32>=vec![1,2,3,4,5];
    display(vector);
}