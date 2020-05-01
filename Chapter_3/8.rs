#[derive(Debug)]
struct Data{
     x:  i32,
    y:String,
    z:Vec<i32>,
}

// Implementing cole trait for a user defined data;
impl Clone for Data{
    fn clone(&self)->Self
    {
        Data{
            x:self.x,
            y:self.y.clone(),
            z:self.z.clone(),
        }
    }
}

fn main()
{   let mut object1= Data{
                       x:4,
                       y:"Object 1".to_string(),
                       z:vec![1,2,3,4,5],
                    };
   //Created copy of object1
   let object2= object1.clone();     
   
   //x of Object 2 is changed
   object1.x=2;
   /* Below, x of two different objects are created, and becuause both are deep copy of each other,
      changing value of one does not affect other.
   */
   println!("\nx of object1 -> {} \nx of object2 -> {}\n",object1.x,object2.x );
   
}