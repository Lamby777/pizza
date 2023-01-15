// Subcommand stuff

use users::get_current_uid;

use crate::{bash, all_users, User};

pub fn init() {
	let res = bash!(include_str!("sh/init.sh"));
	println!("\nCyPatrina init script complete!");

	if res.is_err() {
		println!("Failed to complete CyPatrina init script...");
	} else {
		println!("Please ensure none of its changes caused you to lose points...");
	}
}

pub fn passwd(args: &[String]) {
	let users = all_account_users();

	let em =	args.len()	== 2	&&
					args[0]		== "em";
	
	let new_pass = args.last().unwrap();
	let current_uid = get_current_uid();

	let mut errors = 0u16;

	for user in users {
		if em && (user.uid() == current_uid) { continue };

		let name = user.name().to_string_lossy();

		let cmd = format!("sudo passwd {} <<< \"{}\"$'\n'\"{}\"", name, new_pass, new_pass);

		if bash!(cmd).is_err() {
			errors += 1;
		}
	}

	if errors > 0 {
		println!("{} passwords had issues being updated...", errors);
	} else {
		println!("Passwords updated successfully!");
	}
}

fn all_account_users() -> impl Iterator<Item = User> {
	(unsafe { all_users() }).filter(|user| {
		user.uid() >= 1000 &&	// system users
		user.uid() != 65534		// nobody
	})
}
