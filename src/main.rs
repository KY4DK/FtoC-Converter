use std::io;
fn main() {
	 println!("Convert Fahrenheit to Celsius!");
		loop {
	    println!("INPUT TEMP IN FAHRENHEIT");
	    let mut f = String::new();
		io::stdin()
	        .read_line(&mut f)
		    .expect("Failed to read line");
		
	    let f: f32 = match f.trim().parse()
		{
		    Ok(num) => num,
			Err(_) => continue,
		};
		let x :f32 = 5.0;
		let x :f32 = x / 9.0;
		let c = ({f}-32.0) * (x);
			println!("The temperature in Celsius is: {c}");		
			}
}