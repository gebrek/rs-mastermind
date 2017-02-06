// use std::fmt;

// #[derive(Debug)]
// struct MinMax(i64, i64);

// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // use `self.number` to refer to each positin data point.
//         write!(f, "({}, {})", self.0, self.1)
//     }
// }

// #[derive(Debug)]
// struct Point {
//     x: f64,
//     y: f64,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// #[derive(Debug)]
// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let List(ref vec) = *self;
//         try!(write!(f, "["));
//         for (count, v) in vec.iter().enumerate() {
//             if count != 0 {
//                 try!(write!(f, ", "));
//             }
//             try!(write!(f, "{}", v));
//         }

//         write!(f, "]")
//     }
// }

// struct City {
//     name: &'static str,
//     lat: f32,
//     lon: f32,
// }

// impl fmt::Display for City {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
//         let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

//         write!(f,
//                "{}: {:.3}°{} {:.3}°{}",
//                self.name,
//                self.lat.abs(),
//                lat_c,
//                self.lon.abs(),
//                lon_c)
//     }
// }

// #[derive(Debug)]
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,
//                "RGB ({}, {}, {}) 0x{:02x}{:02x}{:02x}",
//                self.red,
//                self.blue,
//                self.green,
//                self.red,
//                self.blue,
//                self.green,
//         )
//     }
// }

// #[derive(Debug)]
// struct Matric(f32, f32, f32, f32);

// fn main() {
//     let minmax = MinMax(0, 14);
//     println!("Compare structures:");
//     println!("Display: {}", minmax);
//     println!("Debug: {:?}", minmax);

//     let big_range = MinMax(-300, 300);
//     let small_range = MinMax(-3, 3);

//     println!("The big range is {big} and the small is {small}",
//              small = small_range,
//              big = big_range);

//     let point = Point { x: 3.3, y: 7.2 };
//     println!("Compare points:");
//     println!("Display: {}", point);
//     println!("Debug: {:?}", point);

//     let v = List(vec![1, 2, 3]);
//     println!("Compare lists:");
//     println!("Display: {}", v);
//     println!("Debug: {:?}", v);

//     for city in [City {
//                      name: "Dublin",
//                      lat: 53.347778,
//                      lon: -6.259722,
//                  },
//                  City {
//                      name: "Oslo",
//                      lat: 59.95,
//                      lon: 10.75,
//                  },
//                  City {
//                      name: "Vancouver",
//                      lat: 49.25,
//                      lon: -123.1,
//                  }]
//         .iter() {
//         println!("{}", *city);
//     }
//     for color in [Color {
//                       red: 128,
//                       green: 255,
//                       blue: 90,
//                   },
//                   Color {
//                       red: 0,
//                       green: 3,
//                       blue: 254,
//                   },
//                   Color {
//                       red: 0,
//                       green: 0,
//                       blue: 0,
//                   }]
//         .iter() {
//         // Switch this to use {} once you've added an implementation
//         // for fmt::Display
//         println!("{}", *color)
//     }
// }
