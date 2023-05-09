use std::fmt::Display;
use std::collections::HashMap;

#[derive(Debug)]
struct Contact {
    name: String,
    email: String,
}

fn generic_display<T: Display>(item: T) {
    println!("{}", item);

}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T> (T, T);

enum Option<T> {
    Some(T),
    None
}



fn main() {
   let a: &str = "42";
   let b: i64 = 42;

   generic_display(a);
   generic_display(b);

   let (x, y) = (4i16, 2i64);

   let point: Point<i64> = Point {
    x: x.into(),
    y
   };

   let imported_contacts = vec![
    Contact {
        name: "Kliment".to_string(),
        email: "kliment@gmail.com".to_string(),
    },
    Contact {
        name: "Mimoza".to_string(),
        email: "mimoza@gmail.com".to_string(),
    }
   ];
   println!("{:?}", imported_contacts);

   let unique_contacts: HashMap<String, Contact> = imported_contacts
   .into_iter()
   .map(|contact| (contact.email.clone(), contact))
   .collect();


println!("{:?}", unique_contacts);
}