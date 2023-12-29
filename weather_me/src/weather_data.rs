use serde::Deserialize;
// https://openweathermap.org/current#fields_json
#[derive(Debug, Deserialize)]
pub struct Coord {
    lon: f64,
    lat: f64 
}
#[derive(Debug, Deserialize)]
pub struct Weather {
    id: i64, 
    main: String,
    description: String, 
    icon: String
}
#[derive(Debug, Deserialize)]
pub struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
    sea_level: Option<f32>,
    grnd_level: Option<f32>
}
#[derive(Debug, Deserialize)]
pub struct Wind {
    speed: f32, 
    deg: f32,
    gust: Option<f32>    
}
#[derive(Debug, Deserialize)]
pub struct Clouds {
    all: f32
}

#[derive(Debug, Deserialize)]
pub struct Rain {
    _1h: i32, 
    _3h: i32
}

#[derive(Debug, Deserialize)]
pub struct Snow {
    _1h: i32, 
    _3h: i32
}

#[derive(Debug, Deserialize)]
pub struct Sys_ {
    #[serde(rename = "type")]
    type_: i32, 
    id: i32, 
    message: Option<String>,
    country: String,
    sunrise: i32,
    sunset: i32    
}
#[derive(Debug, Deserialize)]
pub struct WeatherData{
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,  
    pub visibility: i32, 
    pub wind: Wind,
    pub clouds: Clouds, 
    pub rain: Option<Rain>,
    pub snow: Option<Snow>, 
    pub dt: i32, 
    pub sys: Sys_, 
    pub timezone: i32,
    #[serde(rename = "id")]
    pub city_id: i32, 
    #[serde(rename = "name")]
    pub city_name: String,
    pub cod: i32
}

#[cfg(test)]
mod tests {
    // https://github.com/serde-rs/json
    use serde_json::from_str;
    use super::*;

    #[test]
    fn derserialize_one()
    {
        let json_str = r#"{"coord":{"lon":-90.4869,"lat":38.7812},"weather":[{"id":701,"main":"Mist","description":"mist","icon":"50d"}],"base":"stations","main":{"temp":284.32,"feels_like":283.87,"temp_min":282.63,"temp_max":285.47,"pressure":1021,"humidity":91},"visibility":10000,"wind":{"speed":3.09,"deg":210},"clouds":{"all":100},"dt":1703337969,"sys":{"type":2,"id":2035160,"country":"US","sunrise":1703337449,"sunset":1703371458},"timezone":-21600,"id":4406831,"name":"Saint Charles","cod":200}"#;
        let weather_data: WeatherData = from_str(json_str).expect("Failed to parse JSON");
        assert_eq!(weather_data.coord.lon, -90.4869);
        assert_eq!(weather_data.coord.lat, 38.7812);
        assert_eq!(weather_data.weather[0].id, 701);
        assert_eq!(weather_data.weather[0].main, "Mist");
        assert_eq!(weather_data.weather[0].description, "mist");
        assert_eq!(weather_data.weather[0].icon, "50d");
        assert_eq!(weather_data.base, "stations");
        assert_eq!(weather_data.main.temp, 284.32);
        assert_eq!(weather_data.main.feels_like, 283.87);
        assert_eq!(weather_data.main.temp_min, 282.63);
        assert_eq!(weather_data.main.temp_max, 285.47);
        assert_eq!(weather_data.main.pressure, 1021.0);
        assert_eq!(weather_data.main.humidity, 91.0);
        assert_eq!(weather_data.visibility, 10000);
        assert_eq!(weather_data.wind.speed, 3.09);
        assert_eq!(weather_data.wind.deg, 210.0);
        assert_eq!(weather_data.clouds.all, 100.0);
        assert_eq!(weather_data.dt, 1703337969);
        assert_eq!(weather_data.sys.type_ , 2);
        assert_eq!(weather_data.sys.id , 2035160);
        assert_eq!(weather_data.sys.country , "US");
        assert_eq!(weather_data.sys.sunrise , 1703337449);
        assert_eq!(weather_data.sys.sunset , 1703371458);
        assert_eq!(weather_data.timezone, -21600);
        assert_eq!(weather_data.city_id, 4406831);
        assert_eq!(weather_data.city_name, "Saint Charles");
        assert_eq!(weather_data.cod, 200);
    }

    #[test]
    fn derserialize_simple()
    {
        let json_str = r#"{"lon":-90.4869,"lat":38.7812}"#;
        let _coord: Coord = from_str(json_str).expect("failed to parse");
        assert_eq!(-90.4869, _coord.lon)
    }


}

//text ref - "{\"coord\":{\"lon\":-90.4869,\"lat\":38.7812},\"weather\":[{\"id\":701,\"main\":\"Mist\",\"description\":\"mist\",\"icon\":\"50d\"}],\"base\":\"stations\",\"main\":{\"temp\":284.32,\"feels_like\":283.87,\"temp_min\":282.63,\"temp_max\":285.47,\"pressure\":1021,\"humidity\":91},\"visibility\":10000,\"wind\":{\"speed\":3.09,\"deg\":210},\"clouds\":{\"all\":100},\"dt\":1703337969,\"sys\":{\"type\":2,\"id\":2035160,\"country\":\"US\",\"sunrise\":1703337449,\"sunset\":1703371458},\"timezone\":-21600,\"id\":4406831,\"name\":\"Saint Charles\",\"cod\":200}"