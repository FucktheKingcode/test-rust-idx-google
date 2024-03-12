use anchor_lang::{prelude::borsh::schema_helpers, solana_program::sysvar::recent_blockhashes::RecentBlockhashes};

fn main() {
    println!("Hello, world!");
    let vios = Moto {category:"Sedan".to_string()};
    let speed_vios = vios.speed();
    print_vehicle_info(&vios);
    chec_speed(&vios);
    print_insurance_info_2(&vios);

    let circle= Circle {
        radius: 10.0
    };

    let rec = Rectangle{
        width: 10.0,
        height: 10.0,
    };

    let vec: Vec<Box<dyn Drawable>> = vec![Box::new(circle.clone()), Box::new(rec.clone())];

    draw_static(&circle);
    draw_static(&rec);

    let tri = Triangle{};
    // draw_static(&tri);

    let shapes: Vec<&dyn Drawable> = vec![&circle, &rec];
    draw_dynamic(&shapes);
}

// pub struct Car {
//     category: String
// }

// impl Car {
//     fn get_category(&self){
//         println!("Category: {}", self.category);
//     }
// }
pub struct Bicycle {
    category: String
}

pub struct Moto {
    category: String
}
pub struct Car {
    category: String
}
// impl Moto {
//     fn get_category(&self){
//         println!("Category: {}", self.category);
//     }
// }

pub trait Vehicle {
    fn get_category(&self)-> String;
    fn speed(&self)-> u32;
}

impl Vehicle for Moto {
    fn get_category(&self)-> String {
        self.category.clone()
    }
    fn speed(&self)-> u32 {
        100
    }
}
impl Vehicle for Car {
    fn get_category(&self)-> String {
        self.category.clone()
    }
    fn speed(&self)-> u32 {
        1000
    }
}

fn print_vehicle_info(vehicle: &impl Vehicle) {
    println!("Category: {}, Speed: {}", vehicle.get_category(), vehicle.speed());
}

fn chec_speed<T: Vehicle> (vehicle: &T) {
    if vehicle.speed() > 80 {
        println!("{} is fast !", vehicle.get_category());
    }else {
        println!("{} is slow .", vehicle.get_category());
    }
}

fn chec_speed_2(vehicle: &impl Vehicle) {
    if vehicle.speed() > 80 {
        println!("{} is fast !", vehicle.get_category());
    }else {
        println!("{} is slow .", vehicle.get_category());
    }
}

fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "Car" => Box::new(Car {category: String::from("Car")}),
        _ => Box::new(Moto{ category: String::from("Moto")}),
    }
}

trait Insurable {
    fn insurance_name(&self) -> String;
}

fn print_insurance_info(item: &(impl Vehicle + Insurable)) {
    println!("{} is insured by {}", item.get_category(), item.insurance_name());
}

fn print_insurance_info_2<T: Vehicle + Insurable>(item: &T) {
    println!("{} is insured by {}", item.get_category(), item.insurance_name());
}

impl Insurable for Moto {
    fn insurance_name(&self) -> String {
        "Baor hiem xe may luon dong hanh cung ban".to_string()
    }
}

trait Displayable: Vehicle {
    fn display_info(&self) {
        println!("Vehicle Category: {}, Speed: {} km/h", self.get_category(), self.speed());
    }
}

impl Displayable for Car {}
impl Displayable for Moto {}

fn display<T: Displayable>(item: &T) {
    println!("{:?}", item.display_info());
}

#[derive(Clone)]
pub struct Circle{
    radius: f64,
}

#[derive(Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.");
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}
pub struct Triangle {}

fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}


fn draw_static_circle(shape: &Circle) {
    shape.draw();
}

fn draw_static_rectangle(shape: &Rectangle) {
    shape.draw();
}


fn draw_dynamic(shapes: &[&dyn Drawable]) {
    for shape in shapes {
        shape.draw();
    }
}