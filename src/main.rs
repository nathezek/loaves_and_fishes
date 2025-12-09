use std::io;
use std::io::Write;

fn main() {
    let game_title = r#"
           ██       ██████   █████  ██    ██ ███████ ███████      █████  ███    ██ ██████      ███████ ██ ███████ ██   ██ ███████ ███████ 
           ██      ██    ██ ██   ██ ██    ██ ██      ██          ██   ██ ████   ██ ██   ██     ██      ██ ██      ██   ██ ██      ██      
           ██      ██    ██ ███████ ██    ██ █████   ███████     ███████ ██ ██  ██ ██   ██     █████   ██ ███████ ███████ █████   ███████ 
           ██      ██    ██ ██   ██  ██  ██  ██           ██     ██   ██ ██  ██ ██ ██   ██     ██      ██      ██ ██   ██ ██           ██ 
           ███████  ██████  ██   ██   ████   ███████ ███████     ██   ██ ██   ████ ██████      ██      ██ ███████ ██   ██ ███████ ███████ 
                                                                                                                               
                     ::::. .::::  ::::. .::::  ::::.   ---=--    .=:                             
                    .::::: .::::. ::::: .::::. :::::    .-=:    .=-=:                            
                    .::::: .::::. ::::: .::::. :::::     --.    --.--.                           
                      .:::   :::.  .:::   :::.  .:::    ----   :=-----                           
                    .:. :: .: .:. :. :: .: .:. :. .:   :=--=-  -------                           
                     ::::: .::::. ::::: .::::. :::::   ==----. -------.                          
                      .:::   :::.  .:::   .::.  .:::  .----=-- ------=.                          
                    .:: .: .:. :. :: .: .:. :. :: .:  :------- -------                           
                     .:::: .::::. .:::: .::::. .::::  :----=-- :------                           
                     . ::: . .::. . .:: . .::. . .::  .-------  =---=.                           
                    .::..: .::.:. ::..: .:: :. ::..:   ==---=.  .=--:                            
                    .::::: .::::. ::::: .::::. :::::   :-:.--    .-:                             
                    .::::: .::::. ::::: .::::. :::::    ----     ---                             
                     ::::. .::::  ::::. .::::  ::::.     -=.   .--=-=:   
    "#;
    println!("{}", game_title);

    // start or quit the game //
    wait_for_start();

    let mut has_lunch: bool = false;
    println!(
        r#"
    🧭 The Setting:

        You are Andrew, one of Jesus's disciples, on a remote, grassy hillside overlooking the Sea of Galilee.
        It is late afternoon. The crowd, estimated to be over five thousand people, is tired and hungry after
        listening to Jesus teach all day. There is no town nearby, and the closest supplies are too far away.
    "#
    );

    let mut current_place = 1;
    while current_place != 0 {
        if current_place != 1 {
            print!("\x1B[2J\x1B[1;1H");
        }

        map_game_rooms(current_place, has_lunch);
        let command = get_user_input();
        let (next_place, next_lunch_state) = handle_command(current_place, &command, has_lunch);

        current_place = next_place;
        has_lunch = next_lunch_state;
    }

    println!("Game Over. May your faith guide you.");
}

// ----------------------------------- NEW HELPER FUNCTION ------------------------------------------ //
// Waits for the user to type "start" or press Enter, then clears the screen.
fn wait_for_start() {
    loop {
        print!("\nType 'start' to play or 'quit' to exit: ");
        io::stdout().flush().expect("Failed to flush message");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let command = input.trim().to_lowercase();

        if command == "start" || command.is_empty() {
            // Clear the terminal after "start"
            print!("\x1B[2J\x1B[1;1H");
            break;
        } else if command == "quit" {
            std::process::exit(0); // Exit the program immediately
        } else {
            println!("Invalid command. Please type 'start' or 'quit'.");
        }
    }
}

// ----------------------------------- GET USER INPUT ------------------------------------------ //
fn get_user_input() -> String {
    print!("What do you do? : ");
    io::stdout().flush().expect("Failed to flush message");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    input.trim().to_lowercase().to_string()
}

// ----------------------------------- ENVIROMENTS OF THE GAME ------------------------------- //
fn map_game_rooms(place_id: i32, has_lunch: bool) {
    match place_id {
        1 => {
            println!("\n--- The Hillside ---");
            println!("Thousands of people are here. Jesus stands on the high ground (NORTH).");

            println!("\n[Possible Commands]");
            println!("- **MOVE:** 'go north', 'go south', 'go path'");
            println!("- **INTERACT:** 'pray'");
        }
        2 => {
            println!("\n--- The Market Path ---");
            println!("This dusty path leads toward the distant town. A beggar asks for alms.");

            println!("\n[Possible Commands]");
            println!("- **MOVE:** 'go south'");
            println!("- **INTERACT:** 'give alms', 'go to town'");
        }
        3 => {
            println!("\n--- The Lake's Edge ---");
            println!("A small boy sits alone near the rocks, clutching a small bundle.");

            println!("\n[Possible Commands]");
            println!("- **MOVE:** 'go north'");

            if !has_lunch {
                println!("- **INTERACT:** 'talk to boy', 'get lunch'");
            } else {
                println!("- **INTERACT:** (You have the food. Time to return.)");
            }
        }
        4 => {
            println!("\n--- Jesus's Position ---");
            println!("Jesus looks at you, waiting.");

            println!("\n[Possible Commands]");
            println!("- **MOVE:** 'go south'");

            if has_lunch {
                println!("- **FINISH MISSION:** 'hand over lunch'");
            }
        }
        _ => {
            println!("Game crashed!");
        }
    }
}

// ------------------------------- COMMAND CENTER LOGIC OF THE GAME ------------------------- //
fn handle_command(current_place: i32, command: &str, has_lunch: bool) -> (i32, bool) {
    if command == "quit" {
        println!("Goodbye !");
        return (0, has_lunch);
    }

    let mut new_lunch_state = has_lunch;

    let next_place = match current_place {
        // --------- PLACE 1 LOGIC: HILLSIDE --------- //
        1 => match command {
            "go north" => 4, // Go to Jesus (Room 4)
            "go south" => 3, // Go to Lake's Edge (Room 3)
            "go path" => 2,  // Go to Market Path (Room 2)
            "pray" => {
                println!(
                    "You pause and pray for guidance. A sense of urgency directs you toward the lake."
                );
                1
            }
            _ => {
                println!("I don't understand that command.");
                1
            }
        },

        // --------- PLACE 2 LOGIC: MARKET PATH --------- //
        2 => match command {
            "go south" => 1, // Go back to Hillside (Room 1)
            "give alms" => {
                println!(
                    "You give the beggar the few coins you have. He thanks you, but the larger problem remains."
                );
                2
            }
            "go to town" => {
                println!(
                    "The journey is too long, and time is short. You realize the solution must be here."
                );
                1 // Forced back to the Hillside (Room 1)
            }
            _ => {
                println!("You can only go SOUTH, GIVE ALMS, or consider going to town.");
                2
            }
        },

        // --------- PLACE 3 LOGIC: LAKE'S EDGE --------- //
        3 => match command {
            "go north" => 1, // Go back to Hillside (Room 1)
            "get lunch" | "talk to boy" => {
                // Multiple commands trigger the same action
                if !has_lunch {
                    println!("The small boy offers his five loaves and two fish. You take them.");
                    new_lunch_state = true; // 🔑 UPDATE THE STATE
                } else {
                    println!("The boy smiles. He has given all he has.");
                }
                3
            }
            _ => {
                println!("I don't understand that command. You can only go NORTH.");
                3
            }
        },

        // --------- PLACE 4 LOGIC: JESUS'S POSITION (VICTORY) --------- //
        4 => match command {
            "go south" => 1, // Go back to Hillside (Room 1)
            "hand over lunch" => {
                if has_lunch {
                    println!(
                        "\n✨ You hand the meager lunch to Jesus. He lifts it to the heavens and blesses it."
                    );
                    println!(
                        "The disciples distribute the food. Every person eats and is filled. VICTORY! ✨"
                    );
                    0 // Return 0 to exit the game loop
                } else {
                    println!("Jesus asks, 'Where is the food we are to bless?'");
                    4
                }
            }
            _ => {
                println!("You can only go SOUTH or HAND OVER LUNCH.");
                4
            }
        },

        // --------- UNKNOWN PLACE ID COLLECTOR LOGIC --------- //
        _ => {
            println!("Error: You are in an undefined location.");
            0
        }
    };

    // Return the new room ID and the potentially updated lunch state
    (next_place, new_lunch_state)
}
