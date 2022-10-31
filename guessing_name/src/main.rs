use rand::Rng;
//调用 rand 下的 Rng 随机数生成器
//	:: 类似于借口
fn main(){
    println!("生成随机数");
    
    //生成随机数范围[1, 101),包含1,不包含101
    let int_number = rand::thread_rng().gen_range(1..101);
    println!("生成的1-100随机数是:{}", int_number);
    
    //生成随机数范围[1, 101),包含1,不包含101
    let int_number = rand::thread_rng().gen_range(1..=100);
    println!("生成的1-100随机数是:{}", int_number);

    //生成随机数范围[0, 1)的 f32 小数
    let float_number = rand::thread_rng().gen::<f32>();
    println!("生成0-1的随机小数:{}", float_number);
    
}