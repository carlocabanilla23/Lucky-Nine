extern crate rand;
use rand::Rng;
use std::io;
use std::process;

pub struct Card{
	_value : i32,
	_suit  : String,
	_name : String
}

impl Card{
	pub fn print(&self){
		println!("{}", self._name);
	}
	pub fn clone(&self) -> Card {
		let a = Card {
			_value : self._value.clone(),
			_suit : self._suit.clone(),
			_name : self._name.clone()	
		};
		return a;
	}
}

fn if_num (str : &&str) -> bool {
    for c in str.chars() {
        if c.is_numeric() == false {
            return false;
        }
    }
    return true;
}

fn total(c : &Vec<Card>) -> i32{
	let mut x = 0;
	for i in c {
		if i._value > 10 { x+= 0}
		else { x+= i._value;}
	}
	return x%10; 
}

fn get_rand(x: &Vec<i32>) -> usize {
	let mut rand = rand::thread_rng().gen_range(1,52);
	
	while check_rand_numbers(rand.try_into().unwrap(),&x) == false {
                rand = get_rand(&x);
        }
	return rand; 
}

fn check_rand_numbers(val: i32 ,x: &Vec<i32>) -> bool {
	for i in x {
		if val == *i { return false }
	}
	return true;
}
fn create_deck() -> Vec<Card> {
	let mut deck : Vec<Card> = Vec::new();
	let suit = ["Heart","Diamond","Club","Spade"];

        for s in suit {
                for x in 1..14 {
                let mut card_name = String::new();
                        if x == 11 { card_name = "Jack".to_string() }
                        else if x == 12 { card_name = "Queen".to_string() }
                        else if x == 13 { card_name = "King".to_string() }
                        else if x == 1 { card_name = "Ace".to_string() }
                        else { card_name = x.to_string()}
                        deck.push(
                                Card {
                                        _value : x,
                                        _suit : s.to_string(),
                                        _name : card_name.to_string() + &" of ".to_string() + &s.to_string()
                                });
                }
        }

	return deck;
}

fn main() {
	let deck : Vec<Card> = create_deck();
	let mut player : Vec<Card> = Vec::new();
	let mut cpu : Vec<Card> = Vec::new();
	let rand_num : Vec<i32> = Vec::new();	
	let mut balance = 2000;
	let mut bet = 0;
	let mut input = String::new();

	while input.trim() != "q" || balance == 0 {
		println!("============================================================");
		println!("Your balance is {}",balance);
		println!("Place a Bet.\nPlease enter a number from 1 to {}. Press q to quit",balance);
		println!("============================================================");	
		
		input.clear();
                io::stdin().read_line(&mut input);
		if input.trim() == "q" { process::exit(1)}
			
		if if_num(&input.trim()) {
			bet = input.trim().parse::<i32>().unwrap();
			println!("my bet {}", bet);
		}else { bet = 99999999 }
		while bet > balance {
			println!("============================================================");
			println!("Your balance is {}",balance);
			println!("Please enter a number from 1 to {}. Press q to quit",balance);
			println!("============================================================");

			input.clear();
                	io::stdin().read_line(&mut input).unwrap();
			if if_num(&input.trim()) {
                        	bet = input.trim().parse::<i32>().unwrap();
               		}else { bet = 999999999 }
			if input.trim() == "q" { process::exit(1)}
		}		
		
		cpu.push(deck[get_rand(&rand_num)].clone());
        	player.push(deck[get_rand(&rand_num)].clone());
        	cpu.push(deck[get_rand(&rand_num)].clone());
        	player.push(deck[get_rand(&rand_num)].clone());
		
		println!();
		println!();
		println!("============================================================");
                println!("===============      Your Card(s)        ==================="); 
        	for c in &player {
   	             c.print();
       		}	
        	println!("total {}", total(&player));
		println!("============================================================");
		
		
		input.clear();
		println!();
                println!();
                println!("============================================================");
               	println!("===============      Pick your move     ===================");
		println!("============================================================");
               	println!("1. Enter \"a\" to Draw a Card");
               	println!("2. Enter any key to Show Card");
               	io::stdin().read_line(&mut input).unwrap();

		if input.trim() == "a" {
			player.push(deck[get_rand(&rand_num)].clone()); 
		println!("======   Your Card(s). Enter any key to continue   ======");
                	input.clear();
			for c in &player {
                        	c.print();
                	}
                	println!("total {}", total(&player));
                        io::stdin().read_line(&mut input).unwrap();
		}else {}
		

		if total(&cpu) < 6 { cpu.push(deck[get_rand(&rand_num)].clone()) }

		println!("============================================================");


		println!();
                println!();
		println!("============================================================");
		println!("===========   Result   =====================================");
		println!("============================================================");
		
		println!();
                println!();
                println!("============================================================");
                println!("===============      CPU  Card(s)        ===================");
		for c in &cpu {
			c.print();
        	}
        	println!("total {}", total(&cpu));
                println!("============================================================");

		println!();
                println!();
                println!("============================================================");
                println!("===============      Your  Card(s)       ===================");
		for c in &player {
                        c.print();
                }
                println!("total {}", total(&player));
                println!("============================================================");
		

		if total(&player) > total(&cpu) {
			balance += bet;
			println!("You won !!!");
			println!("Your current Balance is {}" , balance);
		}else if total(&player) < total(&cpu) {
			balance -= bet;
			println!("You Lost !!!");
                        println!("Your current Balance is {}", balance);
			
			if balance <= 0 {
				println!("No more Balance. Please try again");
				process::exit(1);
			}	
		}else {
			println!("Draw !!!");
		}
		cpu.clear();
		player.clear();

	}
}
