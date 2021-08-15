// Esto serían las clases
#[derive(Debug)]
struct Man {
    name: &'static str,
    age: i32
}

#[derive(Debug)]
struct Woman {
    name: &'static str,
    age: i32
}

// Esto es la forma en la que las estructuras de Man y Women implementan el trait
impl Person for Man {
    fn power(&self) -> i32 {
        200
    }
}

// Estos serían métodos específicos de Man
impl Man {
    fn see_football(&self) -> &'static str {
        "El hombre está viendo un partido de fútbol"
    }
}

impl Person for Woman {
    fn power(&self) -> i32 {
        120
    }
}

// Estos serían métodos específicos de Woman
impl Woman {
    fn go_shopping(&self) -> &'static str {
        "La mujer se va de compras a una tienda de ropa"
    }
}

// Esto equivaldría a una interfaz
trait Person {
    fn power(&self) -> i32;
}

fn main() {
    let man: Man = Man {
        name: "Alberto",
        age: 39
    };
    let woman: Woman = Woman {
        name: "María",
        age: 46
    };
    println!("man = {:?}, woman = {:?}", man, woman);

    // Aquí los métodos del trait
    println!("power of man = {}", man.power());
    println!("power of woman = {}", woman.power());

    // Ejemplo de destructuración de objetos
    let Man { name, age } = Man {
        name: "Pepe",
        age: 67
    };
    println!("El nombre es = {}, y su edad es = {}", name, age);

    // Ejemplo de destructuración de objetos renombrando las propiedades del mismo
    let Woman { name: new_name, age: new_age } = Woman {
        name: "Manuela",
        age: 72
    };
    println!("El nombre es = {}, y su edad es = {}", new_name, new_age);

    // Aquí los métodos específicos de cada estructura
    println!("see_football of man = {}", man.see_football());
    println!("go_shopping of woman = {}", woman.go_shopping());

    // Un ejemplo de un array
    let arr: [u32;3] = [4, 5, 6];
    println!("El array es = {:?}", arr);

    // Un ejemplo de if
    let is_loading = true;
    if is_loading {
        println!("Se está procesando la operación");
    } else {
        println!("La operación ha finalizado");
    }

    // Un ejemplo de if con else if
    let state = "isFinished";
    if state == "isLoading" {
        println!("Se está procesando la operación");
    } else if state == "isFinished" {
        println!("La operación ha sido procesada");
    } else {
        println!("No está definido el estado de la operación");
    }

    // Un ejemplo de if asignado a una variable
    let edad = 18;
    let mayor_de_edad = if edad > 17 { true } else { false };
    println!("Es mayor de edad? = {}", mayor_de_edad);

    // Un ejemplo de bucle infinito con loop
    let mut count = 0;
    loop {
        println!("Esta es una iteracción del bucle loop");
        count += 1;

        if count == 3 {
            break;
        }
    }

    // Un ejemplo de bucle infinito con loop devolviendo un valor
    let mut count2 = 0;
    let _result_from_loop = loop {
        println!("Esta es una iteracción del bucle loop 2");
        count2 += 1;

        if count2 == 3 {
            break count2 + count2;
        }
    };
    println!("Este es el resultado devuelto por el bucle loop = {}", _result_from_loop);

    // Un ejemplo de bucle while
    let mut count3 = 0;
    while count3 < 3 {
        count3 += 1;

        println!("Esta es una iteracción del bucle while con el valor del count = {}", count3);
    }

    // Un ejemplo de bucle for in
    let matriz: [i32; 5] = [12, 24, 64, 73, 20];
    for item in matriz.iter() {
        println!("> {}", item);
    }

    // Un ejemplo de bucle for con notación de rango
    for item in 1..4 {
        println!("> {}", item);
    }

    // Un ejemplo de bucle for con notación de rango, último número incluido y a la inversa
    for item in (1..=4).rev() {
        println!("< {}", item);
    }

    // Una forma de declarar e instanciar una variable de tipo string
    let cadena = String::from("Esta es una cadena");
    println!("{}", cadena);
}
