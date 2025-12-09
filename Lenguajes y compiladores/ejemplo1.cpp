#include <iostream>
#include <thread>
#include <chrono>
using namespace std;

void encender_luz(int id, string color) {
    cout << "Semaforo " << id << " -> " << color << endl;
}

int leerFlujoVehicular(int id) {
   
    return 25;
}

int main() {
    while (true) {
        int flujo = leerFlujoVehicular(1);
        if (flujo > 20) {
            encender_luz(1, "verde");
            this_thread::sleep_for(chrono::seconds(40));
            encender_luz(1, "amarillo");
            this_thread::sleep_for(chrono::seconds(5));
            encender_luz(1, "rojo");
            this_thread::sleep_for(chrono::seconds(30));
        } else {
            encender_luz(1, "verde");
            this_thread::sleep_for(chrono::seconds(20));
            encender_luz(1, "amarillo");
            this_thread::sleep_for(chrono::seconds(5));
            encender_luz(1, "rojo");
            this_thread::sleep_for(chrono::seconds(20));
        }
    }
}