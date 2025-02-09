fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, Test!")
}

#[test]
fn test_variable() {
    let name = "Kurnia Raihan Ardian";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Kurnia Raihan Ardian";
    name = "Rai";
    println!("Hello {}", name);
}

#[test]
fn test_mutable_notused() {
    let mut name = "Kurnia Raihan Ardian";
    // name = 123;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Kurnia Raihan Ardian";
    println!("Hello {}", name);

    let name = 30;
    println!("Hello {}", name);

    name;
}

/*
ini commentar lebih dari satu baris
*/
#[test]
fn comment(){
//  ini commentar
    println!("Hello, comment!");
}

/*
explicit type rust secara otomatis sudah mendeteksi tipe datanya
default tipe data int adalah i32
default tipe data float adalah f32
Saat membuat variable secara implicit (tidak menyebutkan tipe data), maka rust akan memberikan nilai defaultnya
jika bilangan bulat, maka akan menggunakan nilai i32
jika bilangan dengan pecahan decimal, maka akan menggunakan f64
*/
#[test]
fn explicit(){
    let age: i32 = 32;
    println!("{}",age);
}

#[test]
fn number(){
    let a:i32 = 19;
    println!("{}", a);

    let b:f32 = 19.2;
    println!("{}", b);

}
