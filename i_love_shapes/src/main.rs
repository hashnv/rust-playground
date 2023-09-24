trait Area {
    fn area(&self) -> f64;
}

trait Volume {
    fn volume(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Rectangle {
    fn from(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width as f64,
            height: height as f64,
        }
    }
}

#[derive(Debug)]
struct Orthotope {
    dimensions: Vec<f64>,
}

impl Volume for Orthotope {
    fn volume(&self) -> f64 {
        let mut result: f64 = 1.0;
        for n in self.dimensions.clone() {
            result *= n;
        }
        result
    }
}

impl Area for Orthotope {
    fn area(&self) -> f64 {
        let dimensions = self.dimensions.clone();
        let number_of_dimensions = dimensions.len();
        let mut x: f64 = 0.0;
        for n in dimensions {
            x += n;
        }
        ((2.0 as f64).powf(number_of_dimensions as f64 - 1.0)) * x
    }
}


macro_rules! orthotope {
    ($($n:expr),*) => {
        Orthotope { dimensions:vec!($($n as f64),*) }
    };
}

fn main() {
    println!("Area of rectangle is {}", Rectangle::from(2., 2.).area());
    println!("Area of orthotope is {}", orthotope!(1, 2, 1).area());
    println!("Volume of orthotope is {}", orthotope!(2, 2, 2, 2).volume());
    println!("I love shapes.");
}
