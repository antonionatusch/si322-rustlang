enum EstadoConexion {
    Conectado,
    Desconectado,
    Fallido(i32), // Variante con código de error
}

fn main() {
    let estado = EstadoConexion::Fallido(500);

    // Usar match para manejar todas las variantes
    match estado {
        EstadoConexion::Conectado => println!("Conexión activa."),
        EstadoConexion::Desconectado => println!("Conexión cerrada."),
        EstadoConexion::Fallido(codigo) => println!("Error de conexión con código: {}", codigo),
    }

    // Usar if let para manejar solo una variante
    if let EstadoConexion::Fallido(codigo) = estado {
        println!("El error es: {}", codigo);
    }
}
