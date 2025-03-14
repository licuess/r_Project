// 这个函数仅当目标系统是 linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    let var_name = println!("You are running linux!");
    var_name
}

// 而这个函数仅当目标系统 **不是** linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
    println!("hello world to zed");
}
