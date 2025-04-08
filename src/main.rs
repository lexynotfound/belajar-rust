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
    println!("Hello {}", name);
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
tipe data float memiliki dua panjangan data yaitu 32-bit dan 64-bit
tipe data integer memiliki panjang data dari 8-bit - 128-bit
Saat membuat variable secara implicit (tidak menyebutkan tipe data), maka rust akan memberikan nilai defaultnya
jika bilangan bulat, maka akan menggunakan nilai i32
jika bilangan dengan pecahan decimal, maka akan menggunakan f64
Isize dan Usize adalah sebuah tipe data yang memiliki panjang data yang mengikuti dari sistem operasi
jika sistem operasinya 64bit maka isize dan usize akan menjadi 64bit

di dalam rust di saaat kita membuat variable secara implicit (tidak menyebutkan tipe data), maka rust
akan menggunakan default typenya, jika bilangan bulat atau integer, maka rust akan menggunakan i32
dan untuk tipe data float maka akan menggunakan tipe data defaultnya yaitu i64

Konversi Tipe Data Number
di dalam rust bisa melakukan sebuah konversi tipe data dari tipe data number
yang dari ukurannya kecil ke ukuran besar, contohnya dari i32 lalu di konversi ke i64 itu bisa di lakukan
warning
jika kita lakukan konversi dari i64 ke i32 maka harus kita perhatikan, karna bisa aja jangkauannya melebihi
kapasitas dari i32, dan itu akan terjadi sebuah overflow, yaitu kondisi dimana
nilai number tidak bisa di tampung oleh tipe data tujuan yang di konversi
contoh kita punya number 1000000 dalam bentuk i32, lalu kita akan melakukan konversi
ke dalam i8 maka itu akan terjadi overflow karna melebihi kapsitasnya
karna i8 tidak bisa menampung nilai tersebut

lalu jika kita ingin melakukan konversi, kita bisa gunakan kata kunci as

*/

/*
Contoh jika ingin mengunakan implicit
maka harus seperti ini
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


*/
#[test]
fn explicit(){
    let age: i32 = 32;
    println!("{}",age);
}

#[test]
fn number_explicit(){
    let a:i32 = 19;
    println!("{}", a);

    let b:f32 = 19.2;
    println!("{}", b);

}

#[test]
fn number(){
    let a = 19;
    println!("{}", a);

    let b = 19.2;
    println!("{}", b);
}

/*
Konversi Tipe Data Number
di dalam rust bisa melakukan sebuah konversi tipe data dari tipe data number
yang dari ukurannya kecil ke ukuran besar, contohnya dari i32 lalu di konversi ke i64 itu bisa di lakukan
warning
jika kita lakukan konversi dari i64 ke i32 maka harus kita perhatikan, karna bisa aja jangkauannya melebihi
kapasitas dari i32, dan itu akan terjadi sebuah overflow, yaitu kondisi dimana
nilai number tidak bisa di tampung oleh tipe data tujuan yang di konversi
contoh kita punya number 1000000 dalam bentuk i32, lalu kita akan melakukan konversi
ke dalam i8 maka itu akan terjadi overflow karna melebihi kapsitasnya
karna i8 tidak bisa menampung nilai tersebut

lalu jika kita ingin melakukan konversi, kita bisa gunakan kata kunci as

*/

#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = a as i32;
    println!("{}", c);

    // ini adalah contoh dari overflow
    /*
    dan dia akan menghasilkan output
    0
    */
    let d: i64 = 100000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

/*
Numeric OPerator
operator + untuk tambah
operator - untuk di kurang
opeartor / untuk bagi
operator * untuk perkalian
operator % untuk modulo
*/

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{}", c);
    let d = a * b;
    println!("{}", d);
    let e = a / b;
    println!("{}", e);
    let f = a % b;
    println!("{}", f);
    let g = a % b;
    println!("{}", g);
}

/*
Augmented Assignments, di dalam rust
ketika kita ingin menggunakan augmented assignments
seperti a += 10;
maka kita harus menjadikan variable tersebut menjadi mutable
sebuah variable yang bisa di ubah nilainya

*/

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

/*
Tipe Data Boolean adalah tipe data yang sederhana yang hanya memiliki nilai true dan false
dan di dalam rust untuk penulisan boolean adalah bool
contoh cara explicit
let a = true; ini adalah boolean, jika kita ingin lebih di pertegas lagi
kita harus melakukan explicit
let b: bool = false;
*/

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b);
}

/*
Comparison Operators atau operator perbandingan yang di mana menghasilkan boolean
comparison operator > ini adalah operator lebih dari
comparison operator < ini adalah operator kurang dari
comparison operator >= ini adalah operator lebih dari sama dengan
comparison operator <= ini adalah operator kurang dari sama dengan
comparison operator == ini adalah operator sama dengan
*/

#[test]
fn comparison_operator(){
    let a = 20;
    let b = 20;

    let result = a <= b;
    println!("{}", result);
}

/*
Operasi Boolean
operator boolean && ini adalah operator dan
operator boolean || ini adalah operator atau
operator boolean ! ini adalah operator kebalikkan
*/

#[test]
fn boolean_operator(){
    let absen = 75;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("{}", lulus_final);
}

/*
Tipe data char adala tipe data karakter
tipe data char di dalam rust di buat menggunakan petik satu
*/

#[test]
fn char_type(){
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

/*
Tipe data compound
Tipe data tuple adalah tipe data kumpulan lebih dari satu tipe data
untuk jumlah data di tuple sudah final, artinya tipe datanya tidak bisa berkurang ataupun di tambahkan lagi
jika kita membuat tuple dengan total ada 3 data, maka tidak akan bisa di ubah  lagi jumlah data dan juga tipe datanya
untuk membuat tuple, kita bisa gunakan () tanda kurung
contoh penulisan
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
}
untuk melakukan print atau mengeluarkan outputnya menggunakan {:?} dan di dalamnya ada :? biasanya juga di gunakan untuk debug
*/

#[test]
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
}

/*
Mengakses tuple
untuk mengakses tiap data di tuple, kita bisa gunakan . (titik), lalu diikuti dengan nomor index (lokasi) datanya
index di tuple di mulai dari nomor 0
contoh untuk mengakses data tuple
let a = data.0;
let b = data.1;
let c = data.2;
*/

#[test]
fn tuple_access() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
}

/*
Destructuring tuple adalah sebuah metode untuk mengakses seluruh data tuple dan menyimpannya di dalam variable, dengan tanpa harus
mengakses satu satu seluruh data yang ada di dalam tuple, contoh mengkases satu satu data di dalam tuple
ini adalah contoh mengakses data satu persatu di dalam tuple
    let a = data.0;
    let b = data.1;
    let c = data.2;
dan di dalam destructuring tuple ketika kita tidak butuh data dari salah satu di dalam tuple kita bisa menggunakan
_ (garis bawah)
ini adalah contoh ketika kita tidak butuh data yang ada di dalam tuple
 let (a, b,_) = data;

fn tuple_access_destructuring() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}
*/

#[test]
fn tuple_access_destructuring() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
    let (a, b, c) = data;
    // let (a, b,_) = data;

    println!("{} {} {}", a, b, c);
}

/*
Mutable Tuple
secara default di dalam rust tuple itu adalah tipe data yang tidak bisa di ubah yang biasa di sebut imutable
jika kita ingin mengubah data nilai dari variable maka kita harus melakukan metode mutable,
jika kita ingin mengubah tuple dari imutable menjadi mutable caranya hampir sama dengan kita melakukan mutable di dalam variable biasa
contohnya kita tinggal menambahkan mut setalah let, contohnya
let mut data: (i32, f64, bool) = (10, 10.5, true);
*/

#[test]
fn tuple_mutable(){
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
    let (a, b, c) = data;
    // let (a, b,_) = data;

    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 30.5;
    data.2 = false;
    println!("{:?}", data);
}

/*
Unit
Unit adalah tuple tanpa memiliki nilai apapun, yaitu adalah tuple yang tanpa isi apapun di dalamnya, di panggil dengan nama unit
di tulis dengan ()
hal ini mungkin kelihata tidak ada gunannya, biasanya unit ini di gunakan untuk function-function yang tidak membutuhkan hasil data apapun
contoh tuple unit atau kosong

fn unit() {
println!("Hello");
}
*/
fn unit() {
    println!("Hello");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

/*
Array
array adalah tipe data yang berisi kumpulan data, sama seperti tuple yang berisi data banyak
yang membedakan array dengan tuple adalah array hanya bisa menggunakan satu tipe data di dalamnya,
jika array di dalamnya tipe datanya integer maka hanya bisa integer
untuk membuat array, kita bisa gunkana [] tanda kurung kotak.
array di rust tidak seperti pada javascript ataupun php yang nilai panjangnya array itu dinamis
tetapi array di rust mirip dengan di java dan golang, yang nilai panjangnya harus fix dan sudah di tentukan
contoh penulisan array ada dua
1. explicit
let array: [i32; 5] = [1, 2, 3, 4, 5];
2. unexplicit
let array = [1, 2, 3, 4, 5];
*/

#[test]
fn array(){
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
}

/*
Mengakses Array
untuk mengakses array, sama seperti tuple kita perlu tentukan nomor index yang akan di akses
dimulai dari 0 (nol)
namun cara mengaksesnya berbeda, tidak sama seperti tuple yang di mana tuple untuk mengaksesnya menggunakan . (titik)
maka di array untuk mengakses dengan dengan [] kurung siku
*/

#[test]
fn array_access(){
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b)
}

/*
Mutable array
sama seperti tuple, nilai default dari array itu tidak bisa di ganti, ketika
kita ingin mengganti nilai data di dalam array maka kita harus melakukan imutable seperti tuple

*/

#[test]
fn array_mutable(){
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a,b);

    array[0] = 20;
    array[1] = 40;
    println!("{:?}", array);
}

/*
Panjang Array
Hal yang membedakan dengan tuple, kita bisa mendapatkan jumlah data di array
dengan menggunakan function len() milik dari arraynya
len() itu secara defaultnya adalah usize, nilai dari usize mengikut nilai dari sistem operasi kita
kita bisa menggunakan dua cara
1. explicit
let lenght: usize = array.len();
2. unexplicit
let lenght = array.len();
*/

#[test]
fn array_len(){
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a,b);

    array[0] = 20;
    array[1] = 40;
    println!("{:?}", array);

    let lenght = array.len();
    println!("{:?}", lenght);
}

/*
Two Dimensional Array
Two Dimensional array adalah array dua dimensi atau biasa di sebut dengan array di dalam array
saat membuat array, kita bisa menggunakan tipe apapun di dalam arraynya,
dan di dalam penulisan array dua dimensi harus secara explicit tidak bisa dengan cara unexplicit nanti dia akan muncul syntax error
 let matrix: [[i32; 2]; 2]
*/

#[test]
fn two_dimensional_array(){
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
    println!("{}", MINIMUM);
}

/*
Constant
constant adalah variable immutable menggunakan kata kunci const
yang membedakan const dan let adalah, const tidak mutable, selain itu nilai const harus di deklarasikan
ketika kode program dibuat (bukan di jalankan),
oleh karena itu nilai const tidak bisa hasil dari inputan user atau dari kalkulasi nilai lain yang belum jela hasilnya
untuk menbuat const, wajib di sebutkan tipe datanya secara explicit
nama const di rust harus huruf besar, dan biasanya pemisah katanya jika kita memiliki dua kata kita harus memisahkannya dengan garis bawah (_)
jika kita menulis kodenya tanpa melakukan secara explicit atau menentukan tipe datnaya seperti kode dibawah ini
#[test]
fn constant() {
    const MAXIMUM = 0;
    println!("{}", MAXIMUM);
}
maka akan terjadi error di bawah ini
ini adalah contoh error ketika kita tidak menentukan tipe datanya

error: missing type for `const` item
   --> src/main.rs:615:18
    |
615 |     const MAXIMUM = 0;
    |

dan jika kita melakuakn inisialisasi nilai variable untuk nilai const di dalam rust seperti ini
#[test]
fn constant() {
    const MAXIMUM: i32;

    MAXIMUM = 100;
    println!("{}", MAXIMUM);
}
Maka dia akan error seperti ini
error: free constant item without body
   --> src/main.rs:647:5
    |
647 |     const MAXIMUM: i32;
    |     ^^^^^^^^^^^^^^^^^^-
    |                       |
    |                       help: provide a definition for the constant: `= <expr>;`


jika kita mengetikkannya tanpa uppercase atau lowercase atau camel case dia akan memberikan warning
Constant `maximum` should have an upper case name such as `MAXIMUM`
Constant `maxiMum` should have an upper case name such as `MAXI_MUM`
contoh yang benar adalah
#[test]
fn constant() {
    const MAXIMUM: i32 = 0;
    println!("{}", MAXIMUM);
}
ini adalah membuat const di luar function
const MINIMUM: i32 = 200;
#[test]
fn constant() {
    const MAXIMUM: i32 = 0;
    println!("{}", MAXIMUM);
}
ini contoh penulisan yang benar harus uppercase dan jika kita memiliki dua kata di functionnya harus di pisahkan dengan _ (garis bawah)
const MAXI_MUM
*/
const MINIMUM: i32 = 200;
#[test]
fn constant() {
    const MAXIMUM: i32 = 0;
    println!("{} {}", MAXIMUM, MINIMUM);
}

/*
Variable Scope
variable scope adalah sebuah ruanglingkup di dalam function di kode yang sudah di tetapkan, contohnya
variable const MAXIMUM di dalam fn constant maka dia tidak akan bisa di gunakan di luar funciton lain selain scope fn constants
itu di sebut variable local scope. ini contoh variable local scope
#[test]
fn constant() {
    const MAXIMUM: i32 = 0;
    println!("{} {}", MAXIMUM, MINIMUM);
}

dan jika kita ingin menggunakan global variable scope dia seperti in
const MINIMUM: i32 = 0;
ini contoh global variable scope
#[test]
fn two_dimensional_array(){
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
    println!("{}", MINIMUM);
}

*/

#[test]
fn variable_scope(){
    println!("{}", MINIMUM);

    let kurnia = 1;
    {
        println!("{}", kurnia);
        let rai = 2;
        println!("{}", rai);
    }

    // ini adalah contoh local variable yang di gunakan di luar scopenya
    /*
    println!("{}", rai);
    dia akan error seperti ini
    error[E0425]: cannot find value `rai` in this scope
   --> src/main.rs:718:20
    |
718 |     println!("{}", rai);
    |                    ^^^ not found in this scope

    */
}

/*
Memory Management
Garbage Collection adalah fitur yang banyak di gunakan di dalam bahasa program
untuk melakukan manajemen memory, seoerti java dan golang.
secara berkala garbage collection akan memantau data yang tidak di gunakan lagi di memory, dan lalu akan di hapuskan dari memory secara otomatis
di dalam bahasa pemrograman tanpa garbage collection , biasanya harus melakukan manajemen memory secara manual seperti c/c++
tanpa garbage collection, kita harus mengalokasikan data secara manual di dalam memory, begitu juga ketika sudah tidak di butuhkan lagi
kita harus mengahapusnya secara manual di dalam memory
Rust memiliki pendekatan yang berbeda, rust tidak menggunakan garbage collection, dan juga rust tidak memiliki fitur manual memory management

Stack and heap
Rust membagi data di dalam memory menjadi dalam dua bagian yaitu stack dan heap.
stack adala bagian dimana data disimpan dalam struktur dan tumpukan, last in first out, semua data di stack harus yang fixed size
yang artinya ukuran datanya sudah pasati
Heap berbeda, heap seperti tempat untuk menyimpan data dimana untuk menyimpan data di heap kita akan melakukan request terhadap heap, lalu didalam heap
terdapat memory allocator yang berutjuan  untuk menemukan area kosong untuk menyimpan sebuah data dan mengalokasikan data ke area tersebut.
setalah kita di beri pointer (petunjuk) ke lokasi dimana data itu berada di heap.
Pointer dari eap berukuran fix sized, oleh karena itu pointer akan di simpan di stack
Bayangkan saja heap sebagai sebuah gudang yang dimana di dalam gudang memiliki ukuran yang relevan atau bisa besar ataupun kecil
*/

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("Kurnia");
    println!("{} {}", a, b);
}
fn function_b(){
    let a = 10;
    let b = String::from("Raihan");
    println!("{} {}", a, b);
}

/*
 Drop Funciton
 saat variable keluar dari scopenya, yang artinya tidak bisa di akses lagi, secara otomatis
 Rust akan memanggil drop function
 Drop function adalah funciton untuk menghapus data, sehingga akan di bersihkan daari heap ataupun dari stack framenya
 dan jika rust funciton() sudah selesai dieksekusi, maka funciton() tersebut akan di hapus pula dari Stack Frame
 oleh karena itu, Rust tidak membutuhkan garbage collection ataupun manual memory management
*/

/*
&str dan String
&str (string slice) adalah tipe data string yang bersifat immutable dan memiliki ukuran tetap (fixed size). Umumnya digunakan untuk merepresentasikan referensi ke teks yang sudah ada, seperti literal string.
fn main() {
    let greeting: &str = "Halo, dunia!";
    println!("{}", greeting);
}
Ciri-ciri &str:
Immutable (tidak bisa diubah isinya).

Fixed size (ukuran sudah diketahui di compile-time).

Disimpan di stack.

Biasanya digunakan untuk string literal ("teks").

Tidak membutuhkan alokasi memory tambahan di heap.

 Apa itu String?
String adalah tipe data yang bisa menampung teks dinamis dan dapat diubah/ditambahkan (growable). String disimpan di heap karena ukurannya bisa berubah-ubah.
fn main() {
    let mut message = String::from("Halo");
    message.push_str(", dunia!");
    println!("{}", message);
}

Ciri-ciri String:
Growable (bisa ditambah/diubah).
Contoh: String yang Mutable (Growable)
fn main() {
    let mut name = String::from("Kurnia");

    // Menambahkan string
    name.push_str(" Raihan Ardian");

    // Mengganti karakter tertentu (dengan metode lainnya)
    // name.replace(), name.pop(), name.insert(), dll

    println!("Nama lengkap: {}", name);
}

output
Nama lengkap: Kurnia Raihan Ardian

let mut name → kita buat variabel name yang bisa dimodifikasi (mutable).

String::from("Kurnia") → membuat string baru yang berada di heap.

push_str() → metode untuk menambahkan string ke akhir.
Perbandingan dengan &str:
fn main() {
    let name: &str = "Kurnia";
    // name.push_str(" Raihan Ardian"); ❌ ERROR: method not found
}


Mutable (bisa dimodifikasi jika dideklarasikan dengan mut).

Disimpan di heap, karena ukurannya tidak diketahui di compile-time.

Ideal untuk teks yang dibangun secara dinamis (misalnya dari input user, hasil parsing, dll).

Rust memiliki tipe data text yang fixed size, yaitu &str (string slice), dan yang bisa mengembang ukurannya, yaitu string
&str karena ukurannya yang fixed Size, jadi rust akan meyimpannya di stack, sedangkan String karena bisa mengembang, maka disimpan di dalam heap
maka dari itu &str (string slice) itu fixed size,

String di rust merupakan tipe data text UTF-8, dan bisa berkembang ukurnanya
ketika kita buat dalam bentuk immutable variable, maka string tidak bisa berkembang
namun tetap di disimpannya di heap, walaupun kita buatnya menjadi immutable
ketika kita buat dalam bentuk mutable variable, maka string bisa berkembang di heap
string juga memiliki method / function untuk memanipulasi data, namun perlu di perhatikan
ada metod yang digunakan untuk mengubah datanya sendiri, ada juga method yang di gunakan
untuk mengubah dalam bentuk data baru, tanpa memodifikasi data aslinya

document string slice
https://doc.rust-lang.org/std/primitive.str.html

doc string
https://doc.rust-lang.org/stable/std/string/struct.String.html
*/

#[test]
fn string_slice(){
    let name: &str = "   Kurnia Raihan Ardian   ";
    let trimmed = name.trim();

    println!("{}", name);
    println!("{}", trimmed);

    /*
    String slice itu yang berubah adalah variable usernamenya
    bukan isi dari variablenya
    */
    let mut username = "Rai";
    println!("{}", username);

    username = "Kurnia";
    println!("{}", username);
}

#[test]
fn string_type(){
    let mut name: String = String::from("Kurnia Raihan");
    println!("{}", name);

    name.push_str(" Ardian");
    println!("{}", name);

    let rose = name.replace("Kurnia", "Lexy");
    println!("{}", name);
    println!("{}", rose);

}

/*
Ownership
Rust menggunakan Ownership untuk melakukan data management di memory
ownership adalah salah satu fitur unik di rust yang mungkin jarang ada bahasa pemrograman lain
Ownership wajib di mengerti, karena akan berdampak ke hampir semua fitur di rust
Ownership adalah fitur yang digunakan oleh Rust untuk menjadikan rust menjadi bahasa pemrograman yang aman dalam
mengelola data di memory. tanpa harus adanya fitur garbage collection atau manual memory Management
dan untuk memahami ownership itu membutuhkan waktu untuk mempelajari dan memaami ownershoip

Ownership Rules
Di dalam rust ownership memiliki sebuah aturan di dalamnya
Setiap value di rust harus punya owner (variable pemiliki value)
Dalam satu waktu, hanya boleh ada satu owner
Ketika owner keluar scope, value akan di hapus
*/

// Ownership rules
#[test]
fn ownership_rules(){
    // a tidak bisa diakses disini, karna belum di deklarasikan
    let a = 10; // a bisa di akses mulai dari sini

    { // b tidak bisa diakses disini, karna belum di deklarasikan
        let b = 20; // b bisa di akses mulai dari sini
        println!("{}", b);
    } // scope b selesai, b di hapus, b tidak bisa di akses lagi

    println!("{}", a);
} // scope a selesai, a di hapus, a tidak bisa di akses lagi

/*
Data copy
sesuai aturan di ownership rules, setiap value harus dimiliki oleh satu owner pada satu waktu
ketika kita berinteraksi dengan data, maka data akan dimiliki oleh satu owner
semua data yang bersifat fixed size (yang disimpan di stack), ketika kita tambahkan ke variable berbeda (owner baru),
maka hasilnya adalah data akan di copy, sehingga variable baru (owner baru) akan memiliki data hasil copy dari variable lama (owner lama)
oleh karena itu, tiap data akan memiliki satu owner pada satu waktu
contohnya
#[test]
fn data_copy(){
    let a = 10;
    let b = a; // copy data dari a ke b

    println!("{} {}", a, b);
}
TIDAK terjadi perpindahan ownership.

Data pada a disalin (copy) ke dalam variabel b.

Karena tipe data ini sederhana dan ukurannya kecil, Rust secara default menduplikasi nilai ini tanpa memindahkan ownership.

Apakah mutable (mut) mempengaruhi ownership?
Menambahkan keyword mut pada variabel hanya mempengaruhi apakah nilai variabel bisa diubah atau tidak. Keyword mut tidak mempengaruhi aturan ownership maupun perilaku copy pada Rust:

Variabel mut tidak menyebabkan perpindahan ownership.

Ownership hanya berpindah jika tipenya bukan tipe Copy (seperti String atau Vec) ketika variabel di-assign ke variabel lain tanpa metode khusus seperti .clone().

#[test]
fn data_copy(){
    let a = 10;
    let mut b = a; // copy data dari a ke b

    b = 20;

    println!("{} {}", a, b);
}

Contoh ownership berpindah (tipe Non-Copy):
#[test]
fn ownership_move(){
    let a = String::from("rai");
    let b = a; // ownership berpindah ke b


    // println!("{}", a); <-- ini ERROR karena ownership sudah pindah
    println!("{}", b); // aman digunakan
}
Jika ingin ownership berpindah pada tipe primitif?
Pada dasarnya tipe primitif tidak akan pindah ownership secara otomatis. Namun, jika kamu ingin memindahkan ownership tipe primitif, kamu bisa menggunakan tipe seperti Box, yang membuat tipe primitif tersebut tersimpan di heap:

Contoh ownership tipe primitif dengan Box (heap allocation):
#[test]
fn ownership_box(){
    let a = Box::new(10); // integer di heap
    let b = a; // ownership berpindah ke b (move)
    // println!("a: {}", a); // Error: ownership sudah pindah ke b
    println!("{}", b);
}

Penjelasan:

Variabel a adalah pointer ke heap yang menyimpan angka 10.

Ketika assignment let b = a dilakukan, ownership heap tersebut pindah ke variabel b.

Variabel a menjadi invalid setelah pindah ownership.
Mengapa Box bisa dipindahkan (move)?
Box<T> adalah tipe data khusus, yang merupakan sebuah smart pointer di Rust.

Apa itu Box?
Box adalah tipe data yang menyimpan pointer (alamat memori) ke sebuah lokasi di heap.

Box mengalokasikan memori di heap secara dinamis dan kemudian menyimpan data di heap tersebut.

contohnya
let a = Box::new(5i32);

visual
Stack          Heap
+--------+     +--------+
|   a    | --> |   5    |
+--------+     +--------+

Kenapa Box pindah ownership (bukan copy)?
Walaupun Box menunjuk ke nilai primitif (i32), yang dimiliki langsung oleh variabel a bukanlah nilai 5 tersebut, melainkan pointer ke heap.

Pointer ke heap tidak memiliki trait Copy secara default.

Oleh karena itu, saat kamu assign variabel a ke b:

let b = a;

yang sebenarnya terjadi adalah
Stack sebelum move:

+--------+
|   a    | -----> heap [5]
+--------+

Setelah move:

+--------+
|   b    | -----> heap [5]  (b sekarang punya ownership)
+--------+

|   a    | (tidak valid lagi)
Ownership pointer tersebut berpindah dari variabel a ke variabel b.

Ini menyebabkan a tidak valid lagi setelahnya.

Kenapa integer primitif tidak berpindah ownership?
Berbeda dengan Box, integer primitif seperti i32, u32, dll., disimpan langsung di stack dan memiliki trait Copy.

Integer primitif di-stack kecil ukurannya, murah untuk di-copy.

Oleh karena itu, Rust secara default meng-copy data primitif saat di-assign ke variabel lain.

let x = 5u32;
let y = x; // di-copy otomatis, karena integer punya trait Copy.

Stack setelah copy:

+--------+
|   x    | -> 5
+--------+
|   y    | -> 5
+--------+

Tidak ada perpindahan ownership di sini karena kedua variabel x dan y masing-masing memiliki salinan data sendiri.

Jadi kenapa Box<i32> berpindah ownership padahal isinya primitif?
Jawabannya:

Yang dimiliki oleh variabel tipe Box<T> bukan langsung nilai primitifnya, tapi pointer-nya.

Pointer (alamat memori) tidak memiliki trait Copy.

Karena pointer ini tidak punya trait Copy, maka ketika kamu assign ke variabel lain, ownership pointer tersebut berpindah, bukan di-copy.

// function ini mengambil kepemilikan (ownership) dari alokasi memori heap
fn destroy_box(c: Box<i32>){
    println!("menghancurkan kotak yang berisi {}", c)

    // `c` telah di hancurkan dan di hapus dari memory
}

#[test]
fn boxx(){
    // _Stack_ integer yang di alokasikan di memory stack
    let x = 5u32;

    // *Copy* `x` ke `y` - tidak ada sumber daya yang di pindakan
    let y = x;

    // nilai keduanya bisa di gunakan secara independen
    println!("{} {}", x, y);

    // `a` adalah pointer ke integer yang di alokasikan ke memory _heap_
    let a = Box::new(5i32);

    println!("berisi sebuah {}", a);

    // *Move* `a` ke `b`
    let b=a;
    // alamat sebuah pointer (penunjuk) `a` disalin (bukan datanya) ke `b`
    // keduanya sekarang adalah sebuah pointer yang sama di alokasi heap yang sama, tapi
    //`b` sekarang adalah pemilikinya

    // Error! `a` can no longer access the data , because it no longer owns the
    // heap memory
    // yang berarti error itu menandakan bahwa variable dari a itu sudah bukan lagi pemiliknya
    //println!("berisi sebuah: {}", a);
    // TODO ^ Try uncommenting this line / coba untuk hapus komentarnya

    // fungsi ini mengambil ownership atau kepemilikan dari alokasi heap memori yang berasal dari `b`
    destroy_box(b);
    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}

https://doc.rust-lang.org/rust-by-example/scope/move.html

diagram
Stack:                     Heap:
+--------+                +---------+
|   a    | -------------->|   5     |
+--------+                +---------+

// Ownership dipindah (move)
let b = a;

Stack:                     Heap:
+--------+                 +---------+
|   b    | --------------->|   5     |
+--------+                 +---------+
|   a    | (invalid)       (owned by b)
+--------+

// Setelah destroy_box(b) selesai:

Stack:                     Heap:
+--------+                +---------+
|   b    | (invalid)      |  (deleted)
+--------+                +---------+

*/

#[test]
fn data_copy(){
    let a = 10;
    let mut b = a; // copy data dari a ke b

    b = 20;

    println!("{} {}", a, b);
}

#[test]
fn ownership_move(){
    let a = String::from("rai");
    let b = a; // ownership berpindah ke b


    // println!("{}", a); <-- ini ERROR karena ownership sudah pindah
    println!("{}", b); // aman digunakan
}

#[test]
fn ownership_box(){
    let a = Box::new(10); // integer di heap
    let b = a; // ownership berpindah ke b (move)
    // println!("a: {}", a); // Error: ownership sudah pindah ke b
    println!("{}", b);
}

// function ini mengambil kepemilikan (ownership) dari alokasi memori heap
fn destroy_box(c: Box<i32>){
    println!("menghancurkan kotak yang berisi {}", c)

    // `c` telah di hancurkan dan di hapus dari memory
}

#[test]
fn boxx(){
    // _Stack_ integer yang di alokasikan di memory stack
    let x = 5u32;

    // *Copy* `x` ke `y` - tidak ada sumber daya yang di pindakan
    let y = x;

    // nilai keduanya bisa di gunakan secara independen
    println!("{} {}", x, y);

    // `a` adalah pointer ke integer yang di alokasikan ke memory _heap_
    let a = Box::new(5i32);

    println!("berisi sebuah {}", a);

    // *Move* `a` ke `b`
    let b=a;
    // alamat sebuah pointer (penunjuk) `a` disalin (bukan datanya) ke `b`
    // keduanya sekarang adalah sebuah pointer yang sama di alokasi heap yang sama, tapi
    //`b` sekarang adalah pemilikinya

    // Error! `a` can no longer access the data , because it no longer owns the
    // heap memory
    // yang berarti error itu menandakan bahwa variable dari a itu sudah bukan lagi pemiliknya
    //println!("berisi sebuah: {}", a);
    // TODO ^ Try uncommenting this line / coba untuk hapus komentarnya

    // fungsi ini mengambil ownership atau kepemilikan dari alokasi heap memori yang berasal dari `b`
    destroy_box(b);
    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}

/*
Clone
Sekarang kita tahu bahwa data di stack akan di copy sedangkan data di heap akan di pindahkan ownershipnya
lantas bagaimana jika kita juga ingin melakukan copy untuk data di heap?
maka kita harus melakukan clone
Clone adalah sebuah data tiruan yang sama dari data aslinya
String memiliki method clone() untuk melakukan ini
saat kita memanggil method clone() maka method tersebut akan meng-copy data String menjadi string baru
semua tipe data yang disimpan di heap di rust memiliki method clone()
contohnya
fn clone(){
let name1 = String::from("rai);
let name2 = name1.clone();
println!("{}{}", name1, name2);
}
perlu di ingat jika menggunakan clone ini prosess dari clone tersebut akan lebih berat, karna dia akan membuat data yang sama,
jika data awalnya sebesar 10 mega, maka data yang baru di buat nanti akan memiliki besar yang sama
*/

#[test]
fn clone(){
    let name1 = String::from("rai");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

/*
If Expression
if expression di gunakan unutk perkondisian percabangan, kalau nilainya true maka akan di eksekusi
dan penulisan if di dalam rust hampir sama dengan golang
*/

#[test]
fn if_expression(){
    let value = 9;

    if value >= 8{
        println!("{}", value);
    } else {
        println!("Not good");
    }
}

/*
Else If Expression
*/
#[test]
fn elseif_expression(){
    let value = 9;

    if value >= 8{
        println!("{}", value);
    } else if value >= 6 {
        println!("Not Bad");
    } else if value >= 3{
        println!("So bad");
    } else {
        println!("Very bad");
    }
}

/*
Let Statement
If di rust adalah sebuah expression, artinya bisa menghasilkan value dan bisa
digunakan dengan Let Statement untuk mengisi data di variable
ini sangat berguna sehingga kita tidak perlu memasukkan nilai variable terpisah dengan deklarasi variablenya
ini contoh let statement yang rumit atau manual
#[test]
fn letstatement(){
    let value = 9;
    let result: &str;

    if value >= 8{
        result = "Good"
    } else if value >= 6 {
        result = "Not Bad"
    } else if value >= 3{
        result = "Bad"
    } else {
        result = "Very Bad"
    }
    println!("{}", result);
}
kita bisa menggunakan secara otomatis dengan if expression di gabung dengan let statement
#[test]
fn letstatement(){
    let value = 9;
    let result = if value >= 8{
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "very Bad"
    };
    println!("{}", result);
}
*/

#[test]
fn letstatement(){
    let value = 9;
    let result = if value >= 8{
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "very Bad"
    };
    println!("{}", result);
}
/*
Loop
Setiap bahasa pemrograman biasanya memiliki fitur untuk melakukan perulangan
Rust mendukung bebecarap cara untuk melakukan perulangan, pertama kita akan bahasa tentang loop
Loop merupakan perinta di Rust digunakan untuk melakukan perulangan terus-menerus, sampai kita memerintahkannya untuk berhenti
jika kita tidak memerintahkan untuk berhenti, maka loop tidak akan berhenti melakukan perulangan
Unutk menghentikan perulangan, kita bisa menggunakan perintab break
Selain break, ada juga perintah continue, yang artinya  menghentikan perulangan saaat ini, dan langsung di lanjutkan
ke perulangan berikutnya

*/
#[test]
fn loop_expression(){
    let mut counter = 0;
    loop{
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0{
            continue;
        }

        println!("Counter {}", counter);
    }
}

/*
Loop Reutn Value
Sama seperti if expression, di loop juga bisa mengembalikan sebuah nilai
*/

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 100 {
            break counter *2;
        }
    };

    println!("Counter {}", result);
}

/*
Loop Label
Terkadang kita sering membuat Loop didalam loop, dan ketika ingin menghentikan loop paling atas dari loop
yang ada di dalam, maka hal itu tidak bisa di lakukan
loop memiliki fitur label, dimana kita bisa memberi nama pada loop
keuntungannya memberi label pada loop adalah, kita bisa menghentikan loop yang ingin kita hentikan dengan cara menyebutkan nama Labelnya
*/

#[test]
fn loop_label(){
    let mut counter = 0;
    'outer: loop {
        let mut i = 1;
        loop {
            if counter > 100 {
                break 'outer;
            }

            println!("{} X {} = {} ",counter, i, counter * i);
            i += 1;
            if i > 100 {
                break;
            }
        }
        counter += 1;
    }

}

/*
While loop adalah jenis perulangan dimana memiliki kondisi
jika kondisi masih terpenuhi, maka perulangan akan di lanjutkan
namun jika perulangan tidak terpenuhi, maka perulangan akan di hentikan
while loop mirip seperti loop, bisa di hentikan menggunakan break dan continue
dan di dalam while loop kalian bisa juga menggunakan break ataupun continue
*/
#[test]
fn while_loop(){
    let mut counter = 0;
    while counter < 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        counter += 1;
    }
}

/*
Iterasi Array
Salah satu yang biasa kita lakukan ketika menggunakan array adalah, melakukan pengambilan semua data
di array dari data pertama sampai data terakhir.
Biasanya, kita akan menggunakan while loop, lalu membuat variable untuk mengakses indexnya

For Loop
Di dalam Rust menyediakan cara yang mudah untuk melakukan pengambilan data dari array menggunakan for loop
*/

#[test]
fn array_iteration(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len(){
        println!("{}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value {}", value);
    }
}

/*
Rust memiliki tipe data bernama range
range adalah jarak antara start dan end
range merupakan tipe data collection seperti namanya array, sehingga bisa melakukan pengulangan menggunakan for loop
data range akan di mulai dari start (inclusive) dan di akhiri sebelum end (exlusive)
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}
https://doc.rust-lang.org/stable/std/ops/struct.Range.html
Range inclusive
selain range yang exclusive, rust juga memiliki tipe data Range exclusive
implementasinya berbeda dengan range sebelumnya
https://doc.rust-lang.org/stable/std/ops/struct.RangeInclusive.html
*/

#[test]
fn range(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive(){
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("{}", array[i]);
    }
}

/*
Function
function adalaah kumpulan kode yang memiliki nama, dan kegunaannya bisa di panggil
sebelumnya kita sudah tahu tentang main function, yaitu funciton yang di panggil oleh ketika aplikasi berjalan
untuk membuat function, kita menggunakan kata kunci fn dan di ikuti nama functionnya
tradisi nama function / variable di rust menggunakan format snake_case, yaitu huruf kecil semua dan pemisah kata menggunakan _ (garis bawah)
untuk memanggil function, kita bisa langsung sebutkan nama function-nya diikuti dengan kurung buka dan kurung tutup

*/

fn say_hello(){
    println!("Hello");
}

#[test]
fn test_say_hello(){
    say_hello();
    say_hello();
    say_hello();
}

/*
Parameters
saat kita membuat funciton, kita bisa menambahkan parameter, yaitu variable yang menjadi bagian dari definisi funcitonnya
ketika funciton memiliki parameter, maka kita wajib memberi value untuk parameter tersebut ketika memanggil funcitonnya
beberapa orang memanggil parameter dengan argument, jadi jangan terlalu bingung
parameter di function bisa satu atua lebih, dan tiap parameter bisa menggunakna tipe data apapun yang kita inginkan
*/

fn say_goodbye(first_name: &str, las_name: &str){
    println!("{}, {}", first_name, las_name);
}

#[test]
fn parameters_say_goodbye(){
    say_goodbye("Rai", "Raihan");
    say_goodbye("Raihan", "Raihan");
    say_goodbye("Raihan", "Raihan");
}

/*
Return Value
saat membuat funciton, kadang kita ingin mengembalikan hasil eksekusi yang di lakukan di dalam funciton,
atau bisa sebut return value
jika sebuah function ingin mengembalikan value, kita bisa sebutkan ketika deklarasi funciton menggunakan tanda -> lalu di ikuti dengan tipe data kembalian valuenya
baris eksekusi terakhir di funciton akan dianggap sebagai kembalian value-nya
atau jika kita ingin mengembalikan value sebelum baris eksekusi terakhir, kita bisa gunakan kata kunci return, dan diikuti dengan value
yang akan di kembalikan
*/

// Return Value
fn factorial_loop(n:i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop(){
    let result = factorial_loop(10);
    println!("Result: {}", result);

    let result = factorial_loop(-10);
    println!("Result: {}", result);

}

/*
Recursive Function
Recursion merupakan salah satu metode pemecahan masalah diman sebuah solusi pada masalah tersebut bergantung pada solusi dari masalah
yang lebih kecil yang merupakan bagian dari masalah tersebut.
Rust mengimplementasikan recursion dengan memperbolehkan sebuah fungsi untuk memanggil dirinya sendiri (fungsi itu sendiri)
fungsi yang memanggil fungsi itu sendiri biasanya disebut dengan Recursive Function
Misal kita akan buat dua contoh kasus, pertama kita melakukan println tulisan sebanyak parameter menggunakan recursive function. Kedua kita akan ubah factorial
sebelumnya menjadi recursive function
*/

fn print_text(value: String, times: u32){
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text(){
    print_text(String::from("Hello"), 10);
}

fn factorial_recursive(n:u32) -> u32 {
    if n == 1{
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive(){
    let result = factorial_recursive(10);
    println!("Result: {}", result);
}

/*
Ownership dan Function
setelah kita tahu tentang funciton, kita akan bahas lagi tentang Ownership di function parameter
tipe data yang disimpan di heap, ketika kita kirim sebagai parameter di function, secara otomatis Ownershipnya akan berpindah
ke parameter function yang di panggil
karena Ownershipnya berpindah ke parameter function, secara otomatis setalh funciton selesai di eksekusi, maka owner dan value akan di hapus
dan tidak bisa di gunakan lagi
Namun jenis data yang berada di stack, ketika kita kirim sebagai parameter di funciton maka value akan di copy
#[test]
fn test_hi(){
// kode dibawah ini di copy
    let number = 10;
    print_number(number);
    println!("Number: {}", number);

    // kode dibawah ini akan error karna valuenya sudah di pindah di hi(name)
    //karna ketika tipe data dari heap kalian kirimkan ke parameter otomatis ownershipnya di ganti ke parameter tersebut, ketika parameter tersebut tidak dapat di akses lagi di dalam functionnya otomatis datanya akan hilang
    let name = String::from("Rai");
    hi(name); // Value Moved here
    println!("Name: {}", name); // name di samping ini dia akan error yang di karenkan namenya sudah tidak ada data lagi di dalamnya
}

#Kode dibawah ini merupakan sebuah ownership function
#sebenernya ownership name rai ini di pindahkan kepada parameter ini nmae: String
fn hi(nmae: String){
    println!("Hi, {}", nmae);

}

*/

//Funciton Ownership
fn print_number(number: i32){
    println!("number: {}", number);
}

fn hi(nmae: String){
    println!("Hi, {}", nmae);
}

#[test]
fn test_hi(){
    let number = 10;
    print_number(number);
    println!("Number: {}", number);

    let name = String::from("Rai");
    hi(name); // Value Moved here
    // println!("Name: {}", name); // this code got error because Value used after being moved [E0382]
}



