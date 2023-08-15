fn main() {
    let num: i32 = 999;
    let num2 = 111;


    println!("num = {}", num);
    println!("num2 = {}", num2);
    println!("total = {}", num + num2);
    println!();

    if num + num2 < 1000 {
        println!("total under 1000");
    }else{
        println!("total over 1000");
    }

    for x in 1..10{
        for y in 1..10{
            print!("{num:>0len$} ", num = x*y, len=2);
        }
        println!()
    }
}
