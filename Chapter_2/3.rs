/*
    An array is defined with arbitrary str values. 
    Then that array is converted to vector;
*/


fn main()
{
    let mut arr:[&str;5]=["";5];
    arr[0]="bcvbhdj";
    arr[1]="glmkfd0";
    arr[2]="csdsddf";
    arr[3]="sdfdssfd";
    arr[4]="dfsdfsd";

    let  vector= arr[..].to_vec();

    for word in vector
    {
        println!("{}",word);
    }

   
}