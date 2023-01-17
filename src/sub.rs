// Subcommand stuff

use std::borrow::Borrow;

use users::get_current_uid;

use crate::{bash, all_users, User, LINE_SEPARATOR};

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
	let em =	args.len()	== 2	&&
					args[0]		== "em";
	
	let new_pass = args.last().unwrap();

	let errors = for_each_user(|user| {
		let name = user.name().to_string_lossy();

		let cmd = format!("sudo passwd {} <<< \"{}\"$'\n'\"{}\"", name, new_pass, new_pass);

		bash!(cmd).is_ok()
	}, if em { Some(get_current_uid()) } else { None } );

	if errors > 0 {
		println!("{} passwords had issues being updated...", errors);
	} else {
		println!("Passwords updated successfully!");
	}
}

pub fn list_users() {
	for_each_user(|user| {
		println!("({}) {:?}", user.uid(), user.name());

		true
	}, None);
}

pub fn list_sudo_users() {
	println!("The following users are in the 'wheel' or 'sudo' groups:\n{}", LINE_SEPARATOR);

	for_each_user(|user| {
		let groups = user.groups().unwrap();

		let is_admin = groups.iter().any(|g| {
			let group_name = g.name().to_string_lossy();
			
			// Debian uses "sudo" and Fedora uses "wheel"
			matches!(group_name.borrow(), "sudo" | "wheel")
		});
		
		if is_admin {
			println!("({}) {:?}", user.uid(), user.name());
		}

		true // no errors
	}, None);
}

// returns number of fails
fn for_each_user<C>(action: C, except: Option<u32>) -> u16
	where C: Fn(User) -> bool {

	let users = all_account_users();
	let excepting = except.is_some();

	// don't make this call if there isn't a point anyway
	let current_uid = if excepting { get_current_uid() } else { 0 };

	let mut errors = 0;

	for user in users {
		if excepting && (user.uid() == current_uid) { continue };

		if !action(user) {
			errors += 1;
		}
	}

	errors
}

fn all_account_users() -> impl Iterator<Item = User> {
	(unsafe { all_users() }).filter(|user| {
		user.uid() >= 1000 &&	// system users
		user.uid() != 65534		// nobody
	})
}
