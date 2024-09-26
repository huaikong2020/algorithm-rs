fn print_binary(num:i32) {
    for i in (0..32).rev() {
        print!("{}",if num & (1 << i) == 0 {0} else {1})
    }
    println!();
}
fn return_true() -> bool{
    println!("进入了true函数");
    return true;
}
fn return_false() -> bool{
    println!("进入了false函数");
    return false;
}

//为了统一使用加法器进行加减乘除运算，将负数编码为其相反数取反再加一的形式，本质上是同一剩余类群，只不过取的代表元不同。
//例如四位二进制有符号整数：
    //  -8、-7、-6、-5、-4、-3、-2、-1、0、1、2、3、4、5、6、7
    //  8、9、10、11、12、13、14、15、0、1、2、3、4、5、6、7
fn main(){
    let a = 78;
    println!("{a}");
    print_binary(a);
    println!("===========a==========");

	// 	// 负数
    let b = -6;
    println!("{b}");
    print_binary(b);
    println!("============b===========");
	
	// 	// 直接写二进制的形式定义变量
    let c = 0b1001110;
    println!("{c}");
    print_binary(c);
    println!("===========c================");
	
	// 	// 直接写十六进制的形式定义变量
    let d = 0x4e;
    println!("{d}");
    print_binary(d);
    println!("============d=============");
	
	
}