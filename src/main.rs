
extern crate ThRustGorge;
use ThRustGorge::Language;

fn main() {
    //println!("Hello, world!");
	let mut o = Language::on_create();
	o.get_("perc ney02 
        \n object hello = Hello, World!
        \n object obj1 
        \n print hello
        \n object obj2 = Other Text!
        \n obj1 = obj2
        \n create ney1 { 1.6, 1.8, 1.20 }         
        \n obj1 = ney1
        \n print obj2
        \n print obj1
        \n object obj4 = ney02
        \n print obj4
        \n server serv = 192.168.0.2
        \n server serv_= serv
        \n print serv
        \n print serv_
        \n".to_string());
    dbg!(o.get_value("obj2".to_string()));

    /*
        \n server serv0
        \n server serv1 =127.0.0.1:8080
        \n obj1 = serv1 
        \n obj1 = obj2
        \n object obj3 
        \n obj3 = ney1 
        \n create ney0 [5] 
        \n ney1 = ney0
        \n print ney1
        \n ney02 = obj3
        \n ney1 = { 1.2 1.4 1.9 2.0 -5.6 }
        \n remove obj3
        \n print hello
        \n hello = ney1
        \n print hello
        \n hello =Hello, World!== '\" lasoqw U!@` 2?
        \n print hello 
        \n print obj2
        \n print ney1
        \n print ney02
        \n print ney0
        \n print serv1
        \n print obj4    
    */
}
