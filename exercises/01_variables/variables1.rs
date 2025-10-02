fn main() {
    // TODO: Add the missing keyword.
   let x = 5;

//    let y: u8=900;
let s: & str = "hello";

let tup: (i32,f64,&str)=(500,6.4,"hety");
let (x,y,a)=tup;
println!("The value of y and a is : {y} {}",tup.2);

let a=[2;8];

println!("array elements in a: {:?}",a);


    println!("x has the value {x}");
    println!("s has the value {s}");
    println!("tup has the value {tup:?}");
}
