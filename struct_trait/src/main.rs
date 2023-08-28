//-------------------------
//. Traits and Default Implemetation
// --------------------------------

struct Person {
    citizenship : String,
    name : String,
    age : u8,
    gender : char,
    salary : i32,
}

struct Student {
    name_std:  String,
    age : u8,
    sex : char,
    country: String,
}

trait GeneralInfo{
    fn info(&self) -> (&str, u8, char);

    fn country_info(&self) -> &str;

}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)   
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
    
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.country
    }
}


fn main() {
    let person1 = Person{
        name : String::from("Rakesh Kumar"),
        citizenship: String::from("India"),
        age: 29,
        gender: 'M',
        salary: 100_000,
    };

    let Student1 = Student{
        name_std: String::from("Rakesh Kumar"),
        age: 23,
        sex: 'M',
        country : String::from("IND")
    };

    println!("The basic info iunclude {:?} ", person1.info());
    println!("The basic info for the stuent include {:?}", Student1.info());

}
