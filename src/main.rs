use bindgen::Builder;

extern "C" {
    fn cadder(a: i32, b: i32) -> i32;
}

fn main() {
    let bindings = Builder::default()
        .header("lib.h")
        .rustified_enum(".*") // Allow all enums to be rustified
        .allowlist_function("cadder")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    unsafe {
        let result = cadder(1, 2);
        println!("The result is {}", result);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn add_positive_numbers() {
//         unsafe {
//             assert_eq!(add(1, 2), 3);
//         }
//     }

//     #[test]
//     fn add_negative_numbers() {
//         unsafe {
//             assert_eq!(add(-1, -2), -3);
//         }
//     }

//     #[test]
//     fn add_zero() {
//         unsafe {
//             assert_eq!(add(0, 2), 2);
//             assert_eq!(add(2, 0), 2);
//             assert_eq!(add(0, 0), 0);
//         }
//     }

//     #[test]
//     fn add_positive_and_negative_number() {
//         unsafe {
//             assert_eq!(add(-1, 2), 1);
//             assert_eq!(add(1, -2), -1);
//         }
//     }
// }
