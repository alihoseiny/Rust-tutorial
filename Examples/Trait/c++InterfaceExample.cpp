#include <iostream>


class CouldFlyInterface {
public:
    virtual void fly() = 0;
    virtual void land() = 0;
};

class CouldEatInterface {
public:
    virtual void eat() = 0;
};


class Airplane: public CouldFlyInterface {
public:
    Airplane() = default;

    void fly() {
        std::cout << "Airplane is flying" << std::endl;
    }

    void land() {
        std::cout << "Airplane is landing" << std::endl;
    }
};

class Kaftar: public CouldFlyInterface, CouldEatInterface {
public:
    Kaftar() = default;

    void eat() {
        std::cout << "Kaftar the Kakol Be Sar is eating" << std::etl;
    }

    void fly() {
        std::cout << "Kaftar the Kakol Be Sar is flying" << std::endl;
    }

    void land() {
        std::cout << "Kaftar the Kakol Be Sar is landing";
    }
};

void flyTheAirplanes(Airplane *airplaneArray[], int numOfAirplanes) {
    for (int i = 0; i < numOfAirplanes; i++) {
        airplaneArray[i]->fly();
    }
}

void flyTheKaftars(Kaftar *kaftarsArray[], int numOfKaftars) {
    for (int i = 0; i < numOfKaftars; i++) {
        kaftarsArray[i]->fly();
    }
}

void flyBird(CouldFlyInterface * flyableArray[], int numOfFlyables) {
    for (int i = 0; i < numOfFlyables; i++) {
        flyableArray[i]->fly();
    }
}

/*
 * int main() {
 *     Airplane *Airplane1 = new Airplane();
 *         Airplane *Airplane2 = new Airplane();
 *             Airplane *Airplane3 = new Airplane();
 *                 Airplane *airplanes[3] = {Airplane1, Airplane2, Airplane3};
 *                     flyTheAirplanes(airplanes, 3);
 *                         Kaftar *kakolBeSar1 = new Kaftar();
 *                             Kaftar *kakolBeSar2 = new Kaftar();
 *                                 Kaftar *kakolBeSar3 = new Kaftar();
 *                                     Kaftar * kaftars[3] = {kakolBeSar1, kakolBeSar2, kakolBeSar3};
 *                                         flyTheKaftars(kaftars, 3);
 *                                             return 0;
 *                                             }*/

int main() {
    Airplane *Airplane1 = new Airplane();
    Airplane *Airplane2 = new Airplane();
    Airplane *Airplane3 = new Airplane();
    Kaftar *kakolBeSar1 = new Kaftar();
    Kaftar *kakolBeSar2 = new Kaftar();
    Kaftar *kakolBeSar3 = new Kaftar();
    CouldFlyInterface * flyablesArray[] = {Airplane1, Airplane2, Airplane3, kakolBeSar1, kakolBeSar2,kakolBeSar3};
    flyBird(flyablesArray, 6);
    return 0;
}
