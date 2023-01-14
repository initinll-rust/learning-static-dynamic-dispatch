mod animal_example;
mod vehicle_example;

use animal_example::{Dog, Cat, animal_talk_1, animal_talk_2, animal_1, animal_2};
use vehicle_example::{Sedan, SUV, road_trip_1, road_trip_2, water_land_trip_1, water_land_trip_2, Hovercraft};


fn main() {
    let dog = Dog{};
    let cat = Cat{};

    animal_talk_1(&dog);
    animal_talk_1(&cat);

    animal_talk_2(&dog);
    animal_talk_2(&cat);

    _ = animal_1();
    _ = animal_2();

    let car = Sedan{};
    let suv = SUV{};

    road_trip_1(&car);
    road_trip_2(&suv);

    let hovercraft = Hovercraft{};

    water_land_trip_1(&hovercraft);
    water_land_trip_2(&hovercraft);
}
