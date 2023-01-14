
pub trait LandCapable {
    fn drive(&self) {
        println!("Default is driving");
    }
}

pub trait WaterCapable {
    fn float(&self) {
        println!("Default is floating");
    }
}

pub trait Amphibious: LandCapable + WaterCapable {}

pub struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving");
    }
}

pub struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

pub struct Hovercraft {}

impl Amphibious for Hovercraft {}

impl LandCapable for Hovercraft {
    fn drive(&self) {
        println!("Hovercraft is driving");
    }
}
impl WaterCapable for Hovercraft {
    fn float(&self) {
        println!("Hovercraft is floating");
    }
}

// Static Dispath
pub fn road_trip_1(vehicle: &impl LandCapable) {
    vehicle.drive();
}

pub fn water_land_trip_1(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}

// Dynamic Dispath
pub fn road_trip_2(vehicle: &dyn LandCapable) {
    vehicle.drive();
}

pub fn water_land_trip_2(vehicle: &dyn Amphibious) {
    vehicle.drive();
    vehicle.float();
}