use std::fmt;
use std::convert::From;

struct Temperature {
    temperature: f32
}

impl Temperature {
    fn new(temperature: f32) -> Temperature {
        Temperature { temperature }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} º", self.temperature)
    }
}

struct Celsius {
    celsius: Temperature
}

impl Celsius {
    fn new(temperature: Temperature) -> Celsius {
        Celsius { celsius: temperature }
    }
}

impl From<&Fahrenheit> for Celsius {
    fn from(fahrenheit: &Fahrenheit) -> Celsius {
        let celsius_temp = (fahrenheit.fahrenheit.temperature - 32.0) * 5.0 / 9.0;
        
        Celsius::new(Temperature::new(celsius_temp))
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ºC", self.celsius.temperature)
    }
}

struct Fahrenheit {
    fahrenheit: Temperature
}

impl Fahrenheit {
    fn new(temperature: Temperature) -> Fahrenheit {
        Fahrenheit { fahrenheit: temperature }
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ºF", self.fahrenheit.temperature)
    }
}

impl From<&Celsius> for Fahrenheit {
    fn from(celsius: &Celsius) -> Fahrenheit {
        let fahrenheit_temp = celsius.celsius.temperature * 9.0 / 5.0 + 32.0;
        
        Fahrenheit::new(Temperature::new(fahrenheit_temp))
    }
}

fn main() {
    let fahrenheit_temp: Fahrenheit = Fahrenheit::new(Temperature::new(1.0));
    let celsius_temp: Celsius = Celsius::new(Temperature::new(1.0));
    
    println!("{} fahrenheit to celsius is {}", fahrenheit_temp, Celsius::from(&fahrenheit_temp));
    println!("{} celsius to fahrenheit is {}", celsius_temp, Fahrenheit::from(&celsius_temp));
}