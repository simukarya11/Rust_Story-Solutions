/* 
  -> Display functiontakes a vector as an input nd display its elements
*/

fn display(vector:Vec<i32>)
{
    let size=vector.len();
    let mut i=0;
    for num in vector
    {
      
        if i == size-1
        {
            print!("{}\n ",num );
        }
        else
        {
            print!("{}, ",num );
        }
        i+=1;
    }

}

fn main()
{
    let vector:Vec<i32>=vec![1,2,3,4,5];
    display(vector);
}