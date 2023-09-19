use std::collections::HashMap;


#[derive(Eq, Hash, PartialEq)]
#[derive(Clone, Copy)]

struct Coordinates {
    x: i32, 
    y: i32,
}

fn update_visited_squares<'a>(visited_squares: &'a mut HashMap<Coordinates, i32>, explorer_coordinates: Coordinates, moves_to_fade: &'a mut i32) -> &'a HashMap<Coordinates, i32>{
    visited_squares.insert(explorer_coordinates, 0);

    for (coordinate, age) in &mut *visited_squares {
        if age >= moves_to_fade {
            visited_squares.remove(&coordinate);
        }
    }

    visited_squares
}

fn track_explorer(moves_to_make: i32, moves_to_fade: i32, instructions: String) -> Coordinates{

    let mut explorer_coordinates = Coordinates {x: 3, y: 3};
    let mut explorer_direction = 0;//angle 0-360 where 0 is facing upwards

    let mut visited_squares: HashMap<Coordinates, i32> = HashMap::new(); //where value is moves since square was visited

    let mut rotation: Vec<i32> = vec![];
    for direction in instructions.split(""){
        match direction {
            "R" => rotation.push(90),
            "L" => rotation.push(-90),
            _ => rotation.push(0),          
        };
    }

    let mut moves_made = 0;

    while moves_made < moves_to_make {

        match explorer_direction {
            0 => explorer_coordinates.y += 1,
            90 => explorer_coordinates.x += 1,
            180 => explorer_coordinates.y -= 1,
            270 => explorer_coordinates.x -= 1,
            _ => panic!("Invalid explorer direction")
        }

        moves_made += 1;
    }
    explorer_coordinates
}

fn main() {
    println!("Hello, world!");
}
