// fn take_ownership(bar: &str) {
//     let words = bar.split(" ");
//     for word in words {
//         println!("{}", word);
//     }
// }
// struct Result(u8, u8);
// impl Result {
//     fn display(&self) -> String {
//         format!("Suma: {}, Resta: {}", self.0, self.1)
//     }
// }

// struct Person {
//     nombre: String,
//     edad: u32,
// }
//
// impl Person {
//     fn display(&self) -> String {
//         format!("Nombre: {}, Edad: {}", self.nombre, self.edad)
//     }
// }

enum Estado {
    Activo,
    Inactivo,
    Suspendido,
}

fn imprimir_estado(estado: Estado) {
    match estado {
        Estado::Activo => {
            println!("Toy activo")
        }
        Estado::Inactivo => {
            println!("Toy inactivo")
        }
        Estado::Suspendido => {
            println!("Toy suspendido")
        }
    }
}

// fn sum_sub(a: u8, b: u8) -> Result {
//     Result(a + b, a - b)
// }

fn main() {
    // let foo = String::from("Hello World");
    // take_ownership(&foo);
    // println!("{}", foo);
    // let num1: u8 = 10;
    // let num2: u8 = 5;
    // println!("{}", sum_sub(num1, num2).display());

    // let pepe = Person {
    //     nombre: String::from("Pepe"),
    //     edad: 23,
    // };
    // println!("{}", pepe.display());
    imprimir_estado(Estado::Activo);
    imprimir_estado(Estado::Inactivo);
    imprimir_estado(Estado::Suspendido);
}
