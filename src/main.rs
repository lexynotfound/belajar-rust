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
*/
