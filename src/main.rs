use core::num;
use std::io;
fn main(){
    loop{
        println!("Wellcome to temperature converter");
        println!("1. Convert Celcius to Farhenhit");
        println!("2. Convert Farhenhit to Celcius");
        println!("3: Convert Celsius to Kelvin");
        println!("4: Convert Kelvin to Celsius");
        println!("5: Convert Fahrenheit to Kelvin");
        println!("6: Convert Kelvin to Fahrenheit");
        println!("press 7 to exit.");
        let mut choice=String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice:u32=match choice.trim().parse(){
            Ok(num)=>num,
            Err(_)=> continue,
        };
        match choice {
            
            1=>{
                println!("enter temperature in celcius:");
                let celcius=get_input();
                let fahrehit=celcius_to_fahrenhit(celcius);
                println!("{:.2}°C to {:.2}F:",celcius,fahrehit);
            }
            2=>{
                println!("enter temperature in fahrenhit:");
                let fahrehit=get_input();
                let celcius=fahrenhit_to_celcius(fahrehit);
                println!("{:.2}°F to {:.2}C:",fahrehit,celcius);
            }
            3=>{
                println!("enter temperature in celcius:");
                let celcius=get_input();
                let kelvin=celcius_to_kelvin(celcius);
                println!("{:.2}°C to {:.2}F:",celcius,kelvin);
            }
            4=>{
                println!("enter temperature in kelvin:");
                let kelvin=get_input();
                let celcius=kelvin_to_celcius(kelvin);
                println!("{:.2}°C to {:.2}F:",kelvin,celcius);
            }
            5=>{    
                println!("enter temperature in fahrenhit:");
                let fahrenhit=get_input();
                let kelvin=fahrenhit_to_kelvin(fahrenhit);
                println!("{:.2}°C to {:.2}F:",fahrenhit,kelvin);
            }
            6=>{
                println!("enter temperature in kelvin:");
                let kelvin=get_input();
                let fahrehit=kelvin_to_fahrenhit(kelvin);
                println!("{:.2}°C to {:.2}F:",kelvin,fahrehit);
            }
            7=>{
                println!("exiting");
                break;
            }
            _=>{
                println!("Invalid choice! please try again...");
            }        
        }
        fn get_input()->f64{
            let mut input=String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().parse().expect("please type a number...")
        }
        fn celcius_to_fahrenhit(celsius:f64)->f64{
            celsius*1.8+32.0
        }
        fn fahrenhit_to_celcius(fahrenheit:f64)->f64{
            (fahrenheit-32.0)/1.8
       }
       fn celcius_to_kelvin(celsius:f64)->f64{
            celsius + 273.15
       }
       fn kelvin_to_celcius(kelvin:f64)->f64{
            kelvin - 273.15

       }
       fn kelvin_to_fahrenhit(kelvin:f64)->f64{
            (kelvin - 273.15) * 1.8 + 32.0

       }
       fn fahrenhit_to_kelvin(fahrenheit:f64)->f64{
            (fahrenheit - 32.0) / 1.8 + 273.15
       }
    }
}   