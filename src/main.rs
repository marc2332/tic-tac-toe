use rustyline::error::ReadlineError;
use rustyline::Editor;

fn print_table(table: [[&str; 3] ; 3], indicator: &str){
	println!("\nTurn for '{}': ", indicator);
	for (pos_r,r) in table.iter().enumerate() {

		if pos_r == 0 {
			for _ in r {
				print!("-");
				print!("-");
				print!("-");
				print!("-");
			}
			print!("-");
			println!("");
		}
		for (pos_c, c) in r.iter().enumerate() {

			print!("| {} ",c);
			if pos_c == 2 {
				print!("|");
			}
		} 
		println!("");
		for _ in r {
			print!("-");
			print!("-");
			print!("-");
			print!("-");
		}
		print!("-");
		println!("");
	}
}

fn get_input(msg: &str) -> Result<String,()> {
	let mut rl = Editor::<()>::new();
	let readline = rl.readline(msg);
	match readline {
		Ok(line) => {
			Ok(line)
		},
		Err(ReadlineError::Interrupted) => {
			Err(())
		},
		Err(ReadlineError::Eof) => {
			Err(())
		},
		Err(_) => {
			Err(())
		}
	}
}

fn update_table(row: usize, col: usize, table: [[&'static str; 3] ; 3], indicator: &'static str) -> Result<[[&'static str; 3] ; 3],()> {
	let mut temp_table = table;
	temp_table[row][col] = indicator;
	Ok(temp_table)
}

struct FoundEntity {
	pos_r: usize,
	pos_c: usize,
	player: &'static str
}

fn verify_table(table: [[&'static str; 3] ; 3]) -> Result<bool,String>{
	let mut found = Vec::new();
	for (pos_r,r) in table.iter().enumerate() {
		for (pos_c,c) in r.iter().enumerate() {
			// need to implement diagonal combinations
			if pos_c > 0 && c != &" " {
				if c == &r[pos_c-1] {
					found.push(FoundEntity { 
						pos_c: pos_c-1,
						pos_r: pos_r,
						player: c
					});
				}
			}
			if pos_c < 2 && c != &" " {
				if c == &r[pos_c+1] {
					found.push(FoundEntity { 
						pos_c: pos_c+1,
						pos_r: pos_r,
						player: c
					});
				}
			}
			if pos_r > 0 && c != &" " {
				if c == &table[pos_r-1][pos_c] {
					found.push(FoundEntity { 
						pos_c: pos_c,
						pos_r: pos_r-1,
						player: c
					});
				}
			}
			if pos_r < 2 && c != &" " {
				if c == &table[pos_r+1][pos_c] {
					found.push(FoundEntity { 
						pos_c: pos_c,
						pos_r: pos_r+1,
						player: c
					});
				}
			}
		} 
	}
	println!("found:");
	// print found possible combinations
	for f in found {
		println!("{}{}{}",f.pos_r,f.pos_c, f.player);
	}
	// wip
	Ok(true)
}

fn main() {
	
	let mut current_player = 0;
	let mut current_player_indicator = "X";
	let mut table = [[" "; 3] ; 3];
		
	loop {
		verify_table(table).unwrap();
		print_table(table, current_player_indicator);
		let row_input = get_input("Row ? ");
		let col_input = get_input("Column ? ");
		let row: usize;
		let col: usize;
		match row_input {
			Ok(data) => row = data.parse::<usize>().unwrap(),
			Err(()) => break
		}
		
		match col_input {
			Ok(data) => col = data.parse::<usize>().unwrap(),
			Err(()) => break
		}
		
		table = update_table(row, col, table, current_player_indicator).unwrap();

		if current_player == 0 {
			current_player = 1;
			current_player_indicator = "0";
		} else {
			current_player = 0;
			current_player_indicator = "X";
		}

	}
}
