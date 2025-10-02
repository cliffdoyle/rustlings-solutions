
fn anoda_fun(x:&str){
    println!("{}",x)
}

fn main(){
    println!("1st print");
    let x : &str="xanthi";
    let y=x;
    anoda_fun(x);
     println!("{}",y);

}