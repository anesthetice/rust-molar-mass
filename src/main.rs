use std::{io::stdin, io::stdout, io::Write, collections::HashMap};

fn main() -> () {
    let molar_mass_hashmap : HashMap<&str, f64> = HashMap::from([
        ("H", 1.008),
        ("He", 4.0026),
        ("Li", 6.94),
        ("Be", 9.0122),
        ("B", 10.81),
        ("C", 12.011),
        ("N", 14.007),
        ("O", 15.999),
        ("F", 18.998),
        ("Ne", 20.180),
        ("Na", 22.98977),
        ("Mg", 24.305),
        ("Al", 26.982),
        ("Si", 28.085),
        ("P", 30.974),
        ("S", 32.06),
        ("Cl", 35.45),
        ("Ar", 39.948),
        ("K", 39.0983),
        ("Ca", 40.078),
        ("Sc", 44.9559),
        ("Ti", 47.867),
        ("V", 50.9415),
        ("Cr", 51.9961),
        ("Mn", 54.93804),
        ("Fe", 55.845),
        ("Co", 58.933),
        ("Ni", 58.693),
        ("Cu", 63.546),
        ("Zn", 65.38),
        ("Ga", 69.723),
        ("Ge", 72.630),
        ("As", 74.922),
        ("Se", 78.971),
        ("Br", 79.904),
        ("Kr", 83.798),
        ("Rb", 85.4678),
        ("Sr", 87.62),
        ("Y", 88.90584),
        ("Zr", 91.224),
        ("Nb", 92.90637),
        ("Mo", 95.95),
        ("Tc", 98.0),
        ("Ru", 101.07),
        ("Rh", 102.91),
        ("Pd", 106.42),
        ("Ag", 107.87),
        ("Cd", 112.41),
        ("In", 114.82),
        ("Sn", 118.71),
        ("Sb", 121.76),
        ("Te", 127.6),
        ("I", 126.9),
        ("Xe", 131.29),
        ("Cs", 132.90545),
        ("Ba", 137.327),
        ("La", 138.91),
        ("Ce", 140.12),
        ("Pr", 140.91),
        ("Nd", 144.24),
        ("Pm", 145.0),
        ("Sm", 150.36),
        ("Eu", 151.96),
        ("Gd", 157.25),
        ("Tb", 158.93),
        ("Dy", 162.5),
        ("Ho", 164.93),
        ("Er", 167.26),
        ("Tm", 168.93),
        ("Yb", 173.05),
        ("Lu", 174.97),
        ("Hf", 178.49),
        ("Ta", 180.94788),
        ("W", 183.84),
        ("Re", 186.21),
        ("Os", 190.23),
        ("Ir", 192.22),
        ("Pt", 195.08),
        ("Au", 196.97),
        ("Hg", 200.59),
        ("Tl", 204.38),
        ("Pb", 207.2),
        ("Bi", 208.98),
        ("Po", 209.0),
        ("At", 210.0),
        ("Rn", 222.0),
        ("Fr", 223.0),
        ("Ra", 226.0),
        ("Ac", 227.0),
        ("Th", 232.04),
        ("Pa", 231.04),
        ("U", 238.03),
        ("Np", 237.0),
        ("Pu", 244.0),
        ("Am", 243.0),
        ("Cm", 247.0),
        ("Bk", 247.0),
        ("Cf", 251.0),
        ("Es", 252.0),
        ("Fm", 257.0),
        ("Md", 258.0),
        ("No", 259.0),
        ("Lr", 266.0),
        ("Rf", 267.0),
        ("Db", 268.0),
        ("Sg", 269.0),
        ("Bh", 270.0),
        ("Hs", 277.0),
        ("Mt", 278.0),
        ("Ds", 281.0),
        ("Rg", 282.0),
        ("Cn", 285.0),
        ("Nh", 286.0),
        ("Fl", 289.0),
        ("Mc", 290.0),
        ("Lv", 293.0),
        ("Ts", 294.0),
        ("Og", 294.0),
    ]);
    
    let get_user_input = || -> String {
        let mut input_line : String = String::new();
        print!(">> "); stdout().flush().unwrap();
        stdin().read_line(&mut input_line).unwrap();
        return input_line;
    };

    loop {

        let input_line : String = get_user_input();
        match input_line.trim() {
            "exit" => return,
            "Exit" => return,
            "quit" => return,
            "Quit" => return,
            "stop" => return,
            "Stop" => return,
            _ => (),
        }

        let char_vec : Vec<char> = input_line.chars().collect();

        let mut current_element : String = String::new();
        let mut current_element_coefficient : String = String::new();

        let mut molecule_vector : Vec<Vec<(String, u16)>> = Vec::new(); let empty_vec : Vec<(String, u16)> = Vec::new();  molecule_vector.push(empty_vec.clone());
        let mut mv_index : usize = 0;


        for character in char_vec.iter() {
            if *character == ' ' {
                if !current_element.is_empty() {
                    if current_element_coefficient.is_empty() {current_element_coefficient.push('1');};
                    molecule_vector[mv_index].push((current_element.clone(), current_element_coefficient.parse().unwrap()));
                    current_element.clear();
                    current_element_coefficient.clear();
                }
                molecule_vector.push(empty_vec.clone());
                mv_index += 1;
            }
           
            else if character.is_uppercase() {
                if current_element.is_empty() {
                    ()
                } else {
                    if current_element_coefficient.is_empty() {current_element_coefficient.push('1');};
                    molecule_vector[mv_index].push((current_element.clone(), current_element_coefficient.parse().unwrap()));
                    current_element.clear();
                    current_element_coefficient.clear();
                }
                current_element.push(*character);
            }
            
            else if character.is_lowercase() {
                current_element.push(*character);
            }
            
            else if character.is_numeric() {
                current_element_coefficient.push(*character);
            }

            else {
                match *character {
                    '\n' => (),
                    '\r' => (),
                    _ => eprintln!("Invalid character : {}", character),
                } 
            }
        }
        if !current_element.is_empty() {
            if current_element_coefficient.is_empty() {current_element_coefficient.push('1');};
            molecule_vector[mv_index].push((current_element.clone(), current_element_coefficient.parse().unwrap()));
            current_element.clear();
            current_element_coefficient.clear();
        }

        for vec in molecule_vector {
            if !vec.is_empty() {
                let mut molar_mass : f64 = 0.0;
                let mut molecule : String = String::new();
                for tuple in vec {
                    match molar_mass_hashmap.get(&tuple.0[..]) {
                        Some(val) => {
                            molar_mass += *val * f64::from(tuple.1);
                            molecule.push_str(&tuple.0);
                            if tuple.1 > 1 {molecule = molecule + &tuple.1.to_string();}
                        },
                        None => {
                            eprintln!("Error, could not find {} in the periodic table", tuple.0);
                        },
                    }
                }
                println!("{mol:<12} :   {num} g/mol", mol=molecule, num=molar_mass);
            }
        }

    }

}
