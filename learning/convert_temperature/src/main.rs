fn main() {
    let celsius = 23.0;
    let cel_convert = conver_temperature(celsius, 'f');
    assert_eq!(cel_convert, Some(73.4));
    print!("Test Passed\n");

    let farenheight = 84.0;
    let faren_convert = conver_temperature(farenheight, 'c');
    print!("{:?}", faren_convert);
}

fn conver_temperature(temp: f64, dec:char) -> Option<f64>{
    if dec == 'c'{
        return Some(temp - 32.0 / 1.8);
    } else if dec == 'f'{
        return Some(temp * 1.8 + 32.0);
    } else{
        print!("An incompatable temperature type was provided.");
        return None;
    }

}
