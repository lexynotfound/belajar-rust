fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, Test!")
}

/*
Variable adalah tempat untuk menyimpan data,
cara untuk membuat variable di dalam rust yaitu menggunakan let
lalu di dalam rust ketika sudah membuat variable secara default datanya tidak
bisa di ubah lagi, istilahnya yang di sebut immutable
*/
#[test]
fn test_variable() {
    let name = "Kurnia Raihan Ardian";
    println!("Hello {}", name);
}

/*
Di dalam rust di izinkan untuk mengubah isi dari data di dalamnya
namanya adalah mutable
*/

/*
Dan jika kode kita seperti ini ketika menggunakan mutable
#[test]
fn test_mutable() {
    let mut name = "Kurnia Raihan Ardian";
    name = "Rai";
    println!("Hello {}", name);
}
maka akan muncul warning
value assigned to name is never read
let mut name = "Kurnia Raihan Ardian";
Warning ini berarti variabel name kamu deklarasikan, tetapi tidak digunakan sama sekali. Variabel tersebut dibuat (dan diberi nilai) tapi tidak pernah dipakai dalam kode selanjutnya.

Solusi:
Gunakan variabel tersebut dalam statement selanjutnya, atau hapus variabel jika memang tidak dipakai.
Untuk mengilangkan warningnya harus seperti ini
fn test_mutable() {
    let mut name = "Kurnia Raihan Ardian";
    println!("Hello {}", name);
    name = "Rai";
    println!("Hello {}", name);
}

*/
#[test]
fn test_mutable() {
    let mut name = "Kurnia Raihan Ardian";
    name = "Rai";
    println!("Hello {}", name);
}


/*
Rust adalah bahasa yang static typing, artinya dengan jenis data tertentu,
maka dia tidak akan bisa di ubah menjadi tipe data lainnya,
sebelumnya kita membuat tipe data text atau string, lalu kita ingin mengubah
data nya menjadi number, dan akan menjadi error
*/

// #[test]
// fn static_typing() {
//     let mut name = "Kurnia Raihan Ardian";
//     name = 30;
//     println!("Hello {}", name);
// }

#[test]
fn test_mutable_notused() {
    let name = "Kurnia Raihan Ardian";
    // name = 123;
    println!("Hello {}", name);
}

/*
Di rust, kita bisa membuat variable dengan nama yang sama, dan
variable yang sebelumnya akan tertutup atua yang di sebut shadowing,
dan variable yang di akses adalah variable yang baru, praktek ini tidak di rekomen
karna nanti akan membuat kita menjadi bingung
*/
#[test]
fn shadowing() {
    let name = "Kurnia Raihan Ardian";
    println!("Hello {}", name);

    /*
    Kode dibawah ini adalah kode konsep shadowing, dengan dua variable yang berbeda
    tapi menggunakan dua nama yang sama, karna ketika di jalankan 2 variable ini di dalam dua
    memory yang berbeda, ketika kita membuat variable baru dengan nama yang sama di bawahnya,
    maka variable yang sebelumnya akan tertutup dengan variable yang baru dengan nama yang sama
    */
    let name = 30;
    println!("Hello {}", name);

    /*
    Warning name;
    Warning ini berarti ada baris kode yang hanya memanggil variabel tanpa efek apapun (tidak dimasukkan ke dalam ekspresi apapun seperti fungsi atau statement).

Solusi: Hapus baris tersebut jika tidak digunakan, atau gunakan variabel ini di fungsi yang relevan:
name;
    */
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
Data Type atau tipe data, setiap nilai di rust memiliki tipe data
secara garis beasr rust nembagi tipe data enjadi dua subsets; scalar dan compound

dan scalar tyoe merepresentasikan single value (nilai tunggal), contohnya boolean, char, float, dan char
integer merupakan tipe data number dalam bilangan bulat
float type, yaitu tipe data number dalam desimal,
boolean type merupakan sebua tipe data yang menghasilakn true or flase
char type, yaitu adalah tipe data karater contohnya 'a', 'b'

compund type adalah sebuah tipe data yang memiliki banyak data dan tipe data yang berbeda
tuple type, yaitu kumpulan beberapa data yang berbeda seperti yang berisi float, integer, sama boolean, yang bisa memiliki tipe data yang berbeda
array type, yaitu kumpulan data yang memiliki tipe data yang sama, contonya kalau integer isinya harus integer
*/

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
