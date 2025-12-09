use std::{thread, time};

fn encender_luz(id: u32, color: &str) {
    println!("Semaforo {} -> {}", id, color);
}

fn leer_flujo_vehicular(_id: u32) -> u32 {
    // Simulación: siempre retorna 25 vehículos
    25
}

fn main() {
    loop {
        let flujo = leer_flujo_vehicular(1);
        if flujo > 20 {
            encender_luz(1, "verde");
            thread::sleep(time::Duration::from_secs(40));
            encender_luz(1, "amarillo");
            thread::sleep(time::Duration::from_secs(5));
            encender_luz(1, "rojo");
            thread::sleep(time::Duration::from_secs(30));
        } else {
            encender_luz(1, "verde");
            thread::sleep(time::Duration::from_secs(20));
            encender_luz(1, "amarillo");
            thread::sleep(time::Duration::from_secs(5));
            encender_luz(1, "rojo");
            thread::sleep(time::Duration::from_secs(20));
        }
    }
}
