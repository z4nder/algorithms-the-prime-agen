// Two Crystal Balls
// Queremos saber qual é o ultimo andar que podemos
// soltar nossa bolinha de vidro sem que ela quebre na queda
fn main() {
    let inputs = vec!(
        false, false, false, false, 
        false, false, false, false, 
        true, false, false, true, 
        false, false, true, false
    );

    let reponse = two_crystal_balls(inputs).expect("Result not found");

    println!("Breask floor is {}", reponse);
}

fn two_crystal_balls(inputs: Vec<bool>) -> Option<usize> {
    let jump_amount = (inputs.len() as f32).sqrt().round() as i32;
    let mut floors = jump_amount;

    //Busca no index baseado na raiz da lista para tentar encontrar
    while  floors < inputs.len() as i32{
        if inputs[floors as usize] == true {           
            break;
        }

        floors += jump_amount;
    }

    // Volta para onde estava antes da bola quebrar
    floors -= jump_amount;

    // Busca de forma linear para achar o outro ponto
    let mut count = 0;

    while  count < jump_amount && floors < inputs.len() as i32{
        
        if inputs[floors as usize] == true {           
            return Some(floors as usize);
        }

        floors += floors;
        count += count;
    }

    None
}