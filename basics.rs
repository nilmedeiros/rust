// This is the main function
use std::io;

fn main() {
    // Statements here are executed when the compiled binary is called
	println!("Welcome to my World!");
	let mut vari = String::new();
	io::stdin()
	    .read_line(&mut fuck)
	    .expect("Failed to read line");
	println!("Welcome to my world {} times", vari);
	const MAX_LEVERAGE : f64 = 1.5;
	//float64 is the Rust default because is mode accurated, having double precision, while f32 is just single-precision.
	//floating numbers have IEEE-754 standard
	println!("Warning! You've reached {} Leverage score", MAX_LEVERAGE);
	
	//SHADOWING - changing the type of the same variable by showing only the second one
	//As you cannot turn it mutable, shadowing is different from mut
	println!("I'tell you how rich you gonna be the lenght of these spaces as quantity of zeros");
	let dig = "         ";
	let dig = dig.len();
	println!("You will have {} zeros in your income", dig);
	
//Integer Overflow - when the variable becames outside of the specified integer type range, then it will program to panic. ex.: with u8 variable which range(0 to 255), then variable turn to 256.
//If you compile the code in Release Mode with --release flag then it will be wrapping = turning 256 to 0, 257 to 1, and so on. You can also specify wrapping keyword.

//Boolean type - only 2 possib. values: True or false. it has 1 byte in size.
//They're better used for if expression
//Must use bool keyword. exemple:
    let _t: bool = true;
    let _f: bool = false;
	
//Character type - char is 4 bytes in size and represents Unicode Scalar value, getting a range from U+0000 to U+D7FFF and U+E000 to U+10FFFF inclusive. However the hunan charactermay not match up with the Rust char. 
//char literals ae specified with 'literals', rather than like in "strings"
	let _c = 'z';
	let _z = 'Z';
	
//COMPOUND TYPES:

//(TUPLE) - Fixed lenght: once declared, they cannot grow or shrink in size.
	let _tup: (i32, f64, u8) = (-500, 6.4, 3); //Variable _tup binds to entire tuple, because a tuple is considered a single counpound element
	//To get the individual value out of a tuple, we can use pattern matching called destructuring like below:
	let (_x, _y, _z) = _tup;
	println!("The value of y is: {}", _y);
	//Another way of accessing a tuple element is by using a period (.) followed by the index of the value we want, like below:
	let _neg_five_hundred = _tup.0;
	let _six_point_four = _tup.1;
	let _three = _tup.2;
	println!("The limit of your defict is:{}", _neg_five_hundred);
//[ARRAY] - Unlike a tuple, every element of an array must have the same specified type. And also unlike others languages, Rust arrays have a fixed lenght, like tuples.
	//If you wish to grow or shrink a collection type in size, then you must use a "vector" provided by std library.
	//Exemple when you might use an array rather than a vector is in a program that needs to know tha names of months year, because it will always contain 12 elements:
	let _first_quarter = ["January","February","March"];
	let _group1:[i32;6] = [3,5,6,7,8,23]; //Affer giving a name for the variable binded to array, u must declare inside [] the array's type and the number of elements. Then in another [], declare the elements themself.
	//For an array that contains the same value for each element, the :
	let _elem = [3;5]; //The array named _elem contains 5 elements that will all be set by value 3 initially. It's same as let _elem = [3,3,3,3,3]
	//U can access elements using indexing
	println!("The best of the group 1 was the employee number:{}",_group1[5]);
	//Invalid Array Element access: If the index is greater than or equal to the array lenght, RUST WILL PANIC. Which gives Rust one more different and safer feature rather than many low-level languages (which this check is not done, allowing invalid memomry can be accessed). 
	
//IF EXPRESSIONS also called ARMS:
	let number = 6;
	if number % 4 == 0 {
		println!("{} is divisible by 4",number);
	} else if number % 3 == 0 {
		println!("{} is divisible by 3",number);
	} else {
		println!("{} is not divible by 4, or 3",number);
	} //remember first { means Then, and last } is the end of the block.
	//Rust only executes the block for the first true condition, and once it finds one, it doesn't even check the rest.
	//Unlike another languages Rust won't automatic try convert nonBoolean types to a Boolean. 
	let condition = true;
	//Because IF is an expression, it can be used on the right side of a let statement:
	let number = if condition { 5 } else { 8 } ;
	println!("Printing condition on the right side with value {}", number); //Must put in return with the same type of value
	
//LOOPS :
//loop
	let mut counter = 0;
	let result = loop { //result holds the value returned from the loop
		counter += 1;
		
		if counter == 10 {
			break counter * 2;
		}
	};
	println!("The result is {}", result);
//while
	let mut numb = 3;
	while numb != 0 {
		println!("{}",numb);
		numb -= 1 ; //countdown
	}
	let ar = [10, 20, 30, 40, 50]; //slower use of while in arrays because compiler adds runtime code to perform the conditional check. ; plus, it cans cause program panic if index length is incorrect (ex. indcnt = 4 or 6)
	let mut indcnt = 0;
	while indcnt < 5 {
		println!("The value inside of array is {}",ar[indcnt]);
		indcnt += 1;
	}
//For - more commonly used in Rust, CONCISE and SAFER 
    for element in ar.iter(){
		println!("The value in For array is:{}", element);
	}
	//using for in range instead while on countdown case
	for numb in (1..4).rev(){ //reverse range
		println!("{}!", numb);
	}
	
//String type - allocated on the stack(fixed), but its content is allocated on the heap(flexible and cleanable after owner '}').
    //A String is made up of 3 parts: 1-pointer to the heap memory, 2-lengh in bytes the contents is currently using, and 3-total capacity in bytes that String received from allocator
	let mut s = String::from("Hello"); //creating a mutable String from a string literal using the from function
	//The :: is an operator that allows us to namespace this particular from function under String type rather than using a name like string_from
	s.push_str(",world!"); //appends other literal to String with variable called s.
	println!("{}",s);
	//Rust calls drop function - setting variable free, automatically at the closing "}". This patterrn has a profund impact on the way Rust code is written. Behavior of code can be unexpected in situations when have multiples variables using the data we've allocated on the heap.
	//Move = similar to shallow/deep copy in other lang.
	let s1 = String::from("The world");
	let s2 = s1; //s1 was moved into s2. Copy only the stack part of the String
	println!("{} is mine", s2); //if s1 instead s2, would get an error, because Rust doesn't consider s1 into memory anymore
	//Clone method = copy the String fully (Stack+Heap)
	let s3 = String::from("You've been cloned");
	let s4 = s3.clone(); //stack + heap(content) copied
	println!("s3={}, s4={}", s3, s4); //unlike s1, s4 is now valid.
	//Copy trait = any group of simple scalar values and nothing that requires Heap allocation: integers, true/false boolean, float, char types , and Tuples that contain only copy types. Because they are located on stack only.
	let deb = 666;
	let cop = deb; //deb wasn't moved into cop, but copied.
	println!("deb= {}, cop={}", deb, cop);//because they're copy types the both variables are valid into memory.
	
   //Returning values can also transfer/move ownership. It's possible to return multiple values using a tuple, as example below:
   //let str = String::from("hello");
   //let (_s5, len) = calculate_lenght(str);
   //println!("The lenght of '{}' is {}.", str, len);
//fn calculate_lenght(s: String) -> (String, usize){
	//let lenght = s.len(); 
	//(s, lenght)
//}
//REFERENCES and Borrowing
   //Featured by ampersand '&' the references allows you to refer to some value without taking ownership of it. 
   //Taking as example of tuple function above:
   let len = calculate_lenght(&s2);
   println!("The lenght of '{}' is {}", s2, len);
fn calculate_lenght(s: &String) -> usize { //When References are parameters of a fn then they're called Borrowing.
	s.len() //simple; less complex than using tuples; usefull
} //References are immutable by default.
  //You can turn it as &mut, however only once to a particular piece of data in a particular scope=you can make it &mut again if located out of scope.
  //The benefits of this restriction can prevent 'data races' at compile time- when these 3 behaviors occur(won't compile):
     //2 or more pointers access the same data at the same time;
	 //at least 1 of the pointers is being used to write to the data;
	 //there's no mechanism being used to synchronize access to the data.
   
   //Dangling References - Rust prevents the data will not be dangled. In others languages dangling is allowed:
   //fn dangle() -> &String {
   	//   let s = String::from("ahello");
	 //  &s 
	 //}   ERROR because The reference is inside dangle
//SLICE Type - also doesn't have ownsership. They let you reference a contiguous sequence of elements in a collection rather than the whole collection. SIMILAR TO SPLIT in Python. Example of More complex and bug susceptive:
//fn first_word(s: &String) -> usize { //returns a byte index value into the String parameter
	//let bytes = s.as_bytes(); //as_bytes() method converts the String to an array of bytes
	
	//for (i, &item) in bytes.iter().enumerate() { //returns the index and its reference to the element of each element 
		//if item == b' ' { //if reference==space
			//return i; //returns index position
		//}
	//}
	//s.len() //ifelse returns the lenght of s
//}
    //Slice type example in a Rust solution way:
//Step 1- understanding how Slices index works:
let s = String::from("Slices type");
let leng = s.len();
let slices = &s[0..6]; //can also be written by &[..6]
let typ = &s[7..11]; //also be written by &[7..], or even:
//let typ = &s[7..len];
println!("{} {}", slices, typ);
let _fullsl = &s[0..leng];//as the same as &[..];
println!("A slice of the entire string: {}", _fullsl);
  //IMPORTANT:Slice range indices only valid for UTF-8 character.
//Step 2- Returning &str from a Slice type
fn _first_word(s: &String) -> &str { 
//&str a signature of slice that returns a specific point of the binary of a stringliteral...this is why &str is an immutable reference
	let bytes = s.as_bytes(); //converts the var s in an array of bytes
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' { //b' ' means space
			return &s[0..i];
		}
		
	}
	&s[..] //index and lenght of s
}
   //String slices as Parameter- Defining a function to take a string slice instead of a rference to a String makes the API more useful without losing any funcionality:
   let _my_string = String::from("Useful API");
   //let word = _first_word(&_my_string[..]);
   let _my_string_literal = "My string literal";
   //let word = first_word(&my_string_literal[..]);
	let ret = five();
	println!("The returning value of the function five is: {}", ret);
	let ret2 = plus_one(5);
	println!("The returning value of the function plus_one is: {}", ret2);
	
//FUNCTIONS:
//Parameters - when a function has parameters (), you can provide it with (arguments)=concrete values for those parameters.
//When a function has multiple parameters they don't need to be the same.
	another_function(5, 6); //5 and 6 are Arguments
}
//In function signatures, you must declare the type of each parameters
fn another_function(n:i32, m:u8){ //(Parameters)
	println!("The value of N is {}",n); //Statement;
	println!("The value of M is {}",m); //Statement;
//Rust is an expression-based language that contains and differs Statements from Expressions inside its Function bodies. Other languages do not have this distinction.
//Statements are instructions that perform some action and don't return a value. Ex. let keyword, fn definitions.
	//As statements don't return a value in Rust, you can't assign: let x = (let y = 6);.Although that's possible in other languages which would result in x=y=6. 
//Expressions evaluate to a resulting value and sometimes don't include ; in the end.
//Expressions can be part of statements like below:
	let _a = 5; //Statement
	let b = {  //statementstart //Expressionstart{
		let _a = 3; //Expression
		_a + 1 //Expression
	}; //expressionend} //Statementend;
	println!("The value of b is:{}",b); //Expression
}
//Functions Returning values 
fn five() -> i32 { //must declare the type of value after arrow ->
	5 //expression
}
fn plus_one (ret2: i32) -> i32 {
	ret2 + 1  //expression
}

			//STRUCTS
			

struct Client {
	clientname: String,
	email: String,
	index: u64,
	active: bool,
}

let mut client1 = Client {
	email: String::from("alexander@icloud.com"),
	clientname: String::from("sasha"),
	active: true,
	index: 1
};

