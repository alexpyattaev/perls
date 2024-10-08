fn do_stuff()->Result<i32, String>{
    Ok(42)
}

macro_rules! oh_my {
	() => {
		if let Ok(X) = do_stuff() {println!("Got {X}")}  else {println!("Could not do"); };
	};
}



fn main(){
    const X:&str = "This does not affect the macro in any way, right?";
    oh_my!();
}
