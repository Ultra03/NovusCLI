use std::process::Command;
use std::process;
use std::env;

fn main() {
	//Grabs arguments user inputs
	let args: Vec<String> = env::args().collect();
	//For some reason no arguments are equal to one so this checks if there are no arguments and prints this help message with an error at the end.
	if args.len() == 1 {
		eprintln!("Novus CLI Help\nnvs [command] <query>\n\nsearch [query]\t\t\tSearches for a package in your resporitories\nlist\t\t\t\tLists all packages in your resporitories\ninfo [package]\t\t\tDisplays information of a selected package\ninstall [package]\t\tInstalls a package\nreinstall [package]\t\tReinstalls a package\nremove [package]\t\tRemoves a package\nedit-sources\t\t\tOpens the APT repo editor\nautoremove\t\t\tRemoves unneeded packages (orphans)\nupdate\t\t\t\tUpdate the repo lists\nupgrade\t\t\t\tUpgrade all packages\nfull-upgrade\t\t\tLike upgrade but does more\nclean\t\t\t\tClear the download cache\nhelp\t\t\t\tOpen this help page\nabout\t\t\t\tView legal information\n");
		process::exit(1);
		//For some reason 1 argument is equal to two so this checks if there is 1 argument.
	} else if args.len() == 2 {
		if args[1] == "list" {
			//Runs apt list
			Command::new("apt").arg("list").status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "autoremove" {
			//Runs sudo apt purge --autoremove
			Command::new("sudo").arg("apt").arg("purge").arg("--autoremove").status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "update" {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "upgrade" {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().expect("Something went wrong.");
			//Runs sudo apt upgrade
			Command::new("sudo").arg("apt").arg("upgrade").status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "full-upgrade" {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().expect("Something went wrong.");
			//Runs sudo apt upgrade
			Command::new("sudo").arg("apt").arg("upgrade").status().expect("Something went wrong.");
			//Runs sudo apt full-upgrade
			Command::new("sudo").arg("apt").arg("full-upgrade").status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "clean" {
			//Runs sudo apt autoclean
			Command::new("sudo").arg("apt").arg("autoclean").status().expect("Something went wrong.");
			//Runs sudo apt clean
			Command::new("sudo").arg("apt").arg("clean").status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "help" {
			//Prints help menu.
			println!("Novus CLI Help\nnvs [command] <query>\n\nsearch [query]\t\t\tSearches for a package in your resporitories\nlist\t\t\t\tLists all packages in your resporitories\ninfo [package]\t\t\tDisplays information of a selected package\ninstall [package]\t\tInstalls a package\nreinstall [package]\t\tReinstalls a package\nremove [package]\t\tRemoves a package\nedit-sources\t\t\tOpens the APT repo editor\nautoremove\t\t\tRemoves unneeded packages (orphans)\nupdate\t\t\t\tUpdate the repo lists\nupgrade\t\t\t\tUpgrade all packages\nclean\t\t\t\tClear the download cache\nhelp\t\t\t\tOpen this help page\nabout\t\t\t\tView legal information\n\n");
			process::exit(0);
		} else if args[1] == "about" {
			//Prints about menu.
			println!("About Novus CLI\nCopyright (C) 2019 Polar Development.\nhttp://randomlink.com\n\nCopyright 2019 Polar Team\nLicensed under the Apache License, Version 2.0 (the \"License\");\nyou may not use this file except in compliance with the License./nYou may obtain a copy of the License at\n\nhttp://www.apache.org/licenses/LICENSE-2.0\nUnless required by applicable law or agreed to in writing, software\ndistributed under the License is distributed on an \"AS IS\" BASIS,\nWITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\nSee the License for the specific language governing permissions and\nlimitations under the License.\n\nNovus CLI is made possible by the following software and people: \n\nNikan Radan (SmushyTaco) for writing it\nAPT by Debian: https://packages.debian.org/stretch/apt\nDiego Magdaleno & Diatrus for hosting it on Project Serna: https://sernarepo.com/");
			process::exit(0);
		} else if args[1] == "edit-sources" {
			//Runs sudo apt edit-sources
			Command::new("sudo").arg("apt").arg("edit-sources").status().expect("Something went wrong.");
			process::exit(0);
			//Checks if any of the entries with 2 arguments were entered with only 1 argument.
		} else if args[1] == "search" || args[1] == "install" || args[1] == "reinstall" || args[1] == "remove" || args[1] == "info" {
			//Gives error message if only 1 argument was inputted for something that requires 2 arguments.
			eprintln!("{:?} has two arguments but you only entered one, run \"nvs help\" to check how to properly use {:?}.", args[1], args[1]);
			process::exit(1);
		} else {
			eprintln!("Unknown operation {:?}. Type \"nvs help\" to see the list of available commands.", args[1]);
			process::exit(1);
		}
	} else {
		if args[1] == "search" && args[2] == args[2] {
			//Runs apt search + second argument user inputted.
			Command::new("apt").arg("search").arg(&args[2]).status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "install" && args[2] == args[2] {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().expect("Something went wrong.");
			//Runs sudo apt install
			Command::new("sudo").arg("apt").arg("install").arg(&args[2]).status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "reinstall" && args[2] == args[2] {
			//Runs sudo apt update
			Command::new("sudo").arg("apt").arg("update").status().expect("Something went wrong.");
			//Runs sudo apt reinstall + second argument user inputted.
			Command::new("sudo").arg("apt").arg("reinstall").arg(&args[2]).status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "remove" && args[2] == args[2] {
			//Runs sudo apt purge + second argument user inputted.
			Command::new("sudo").arg("apt").arg("purge").arg(&args[2]).status().expect("Something went wrong.");
			process::exit(0);
		} else if args[1] == "info" && args[2] == args[2] {
			//Runs apt info + second argument user inputted.
			Command::new("apt").arg("info").arg(&args[2]).status().expect("Something went wrong.");
			process::exit(0);
		} else {
			eprintln!("Unknown operation {:?}. Type \"nvs help\" to see the list of available commands.", args[1]);
			process::exit(1);
		}
	}
}








