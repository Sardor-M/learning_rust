fn main (){
    let mut original_string = String::from("Hello");

    // creating a new function to modify the string and get reference to it
    let changed_reference = modify_string(&mut original_string);

    // printing modified reference
    println!("Modified reference: {}", changed_reference);
}

// function which takes the ownership of a string and returns a  reference to it
fn modify_string<'a>(input: &'a mut String) -> &'a mut String {
    // modifing the input string
    input.push_str(", People");
    &input[..] // returning a reference to modified string
}


/* 

Explanation: 

Borrowing:
modify_string funksiyasi `&a mut String` shaklida ozgaruvchi bilan ishlaydi. 
Va bu holat mutable(string) egasi bo'lgan o'tkazib beradi. `input` original stringga 
berilgan ozgaruvchi(mutable)ning o'z ichiga olgan orignal string uchun reference qiladi.

Lifetimes:
`<'a> ` ichida `a` bu hayotiy vaqtini bildiradi. Yani (`&'a str`) funsiyasi tomonidan qaytarilgan refenresiya va `&'a mut String` 
funksiya parameteri bilan birgalikda bir xil hayotiy vaqtini bildiradi.

Ownership:
funskiya chaqirilganda `original_string` egasini boshqa funksiyaga otkazish amalga oshadi. 

*/

