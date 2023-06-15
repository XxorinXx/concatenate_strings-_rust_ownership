fn main() {
    let string1 = String::from("Orin");
    let string2 = String::from("_Hazan");

    let concatenated_string = concatenate_strings(&string1 , &string2);
    println!("{}",concatenated_string);

    
 }

fn concatenate_strings(s1:&String , s2:&String) -> String 
{
    let mut result = String::from(s1); 
    result.push_str(s2);
    result
}
