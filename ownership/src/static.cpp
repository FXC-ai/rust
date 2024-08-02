#include <iostream>
#include <cmath>

struct Cercle {
    uint32_t id;
    float rayon;

    // Static member function
    static Cercle create_cercle(uint32_t id, float rayon) {
        return Cercle(id, rayon);
    }

    // Member function to calculate the area
    float aire() const {
        return rayon * rayon * M_PI;
    }
};

int main() {
    // Call the static member function to create an instance
    Cercle cercle3 = Cercle::create_cercle(3, 5.6);
    // Call the instance method to calculate the area
    std::cout << "Surface du cercle = " << cercle3.aire() << std::endl;
    return 0;
}
