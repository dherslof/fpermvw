use std::error::Error;
use std::fmt;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;

use clap::ArgMatches;
use prettytable::{Row, Table};

use crate::utils;

//------------------------------------------------------------------------------
// Print options
//------------------------------------------------------------------------------

pub enum PrintStyle {
    Simple,
    // Prep for later...(if wanted)
    //Unix,
}

//------------------------------------------------------------------------------
// File Permission struct type
//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct FilePermission {
    dir: bool,
    user: PermValues,
    group: PermValues,
    other: PermValues,
    num: u32,
    name: String,
    symlink: bool,
}

impl fmt::Display for FilePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nFile name:{:?}\nIs directory: {:?}\nUser: {:?}\nGroup: {:?} \nOther: {:?}\n",
            self.name, self.dir, self.user, self.group, self.other
        )
    }
}

//------------------------------------------------------------------------------
// FilePermission associated methods
//------------------------------------------------------------------------------

impl FilePermission {
    fn print(&self, style: PrintStyle) {
        // match statement not needed and can be removed, however at some point maybe
        // more options can be implemented.
        match style {
            PrintStyle::Simple => {
                let mut perm_table = Table::new();
                perm_table.set_titles(row![
                    "File Permissions",
                    "Is directory",
                    "Is symlink",
                    "read",
                    "write",
                    "execute"
                ]);

                perm_table.add_row(row![self.name, self.dir, self.symlink]);
                perm_table.add_row(self.user.as_row("User"));
                perm_table.add_row(self.group.as_row("Group"));
                perm_table.add_row(self.other.as_row("Other"));
                perm_table.printstd();
            }
        }
    }
}

//------------------------------------------------------------------------------
// FilePermission struct associated functions
//------------------------------------------------------------------------------

impl FilePermission {
    // Get other permission bits
    fn perm_other(b: &u8) -> PermValues {
        let mut num: u32 = 0;

        //low nibble 0
        let r = if ((b & 0b000_0100) >> 2) == 1 {
            num = num + 4;
            true
        } else {
            false
        };
        let w = if ((b & 0b000_0010) >> 1) == 1 {
            num = num + 2;
            true
        } else {
            false
        };
        let e = if (b & 0b000_0001) == 1 {
            num = num + 1;
            true
        } else {
            false
        };

        return PermValues {
            read: r,
            write: w,
            execute: e,
            octal_num: num,
        };
    }

    //------------------------------------------------------------------------------

    // Get group permission bits
    fn perm_groups(b: &u8) -> PermValues {
        let mut num: u32 = 0;

        //high nibble 0
        let r = if ((b & 0b010_0000) >> 5) == 1 {
            num = num + 4;
            true
        } else {
            false
        };
        let w = if ((b & 0b001_0000) >> 4) == 1 {
            num = num + 2;
            true
        } else {
            false
        };
        let e = if ((b & 0b000_1000) >> 3) == 1 {
            num = num + 1;
            true
        } else {
            false
        };

        return PermValues {
            read: r,
            write: w,
            execute: e,
            octal_num: num,
        };
    }

    //------------------------------------------------------------------------------

    // Get user permission bits
    fn perm_user(n1l_b: &u8, n0h_b: &u8) -> PermValues {
        let mut num: u32 = 0;

        // low nibble 1
        let r = if (n1l_b & 0b000_0001) == 1 {
            num = num + 4;
            true
        } else {
            false
        };

        //high nibble 0
        let w = if ((n0h_b & 0b000_1000_0000) >> 7) == 1 {
            num = num + 2;
            true
        } else {
            false
        };
        let e = if ((n0h_b & 0b100_1000) >> 6) == 1 {
            num = num + 1;
            true
        } else {
            false
        };

        return PermValues {
            read: r,
            write: w,
            execute: e,
            octal_num: num,
        };
    }

    //------------------------------------------------------------------------------

    fn from_name(file: &String) -> Result<FilePermission, failure::Error> {
        println!("from name func");
        let f = File::open(file)?;
        let metadata = f.metadata()?;

        /* Keep for debugging/testing
        let permissions = metadata.permissions();
        println!("permissions: {:o}", permissions.mode());
        println!("permissions: {:b}", permissions.mode());
        */

        let as_num: u32 = metadata.permissions().mode();
        let as_bytes = as_num.to_be_bytes();

        return Ok(Self {
            dir: utils::directory(file)?,
            user: FilePermission::perm_user(&as_bytes[2], &as_bytes[3]),
            group: FilePermission::perm_groups(&as_bytes[3]),
            other: FilePermission::perm_other(&as_bytes[3]),
            num: as_num,
            name: file.to_string(),
            symlink: utils::symlink(file)?,
        });
    }
}

//------------------------------------------------------------------------------
// Permission values container struct & error
//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PermValues {
    read: bool,
    write: bool,
    execute: bool,
    octal_num: u32,
}

#[derive(Debug)]
pub struct ArgumentErrorLength {
    length: usize,
    expected: u32,
}

impl fmt::Display for PermValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Read: {:?}, Write: {:?}, Execute: {:?}, Octal-number: {} \n",
            self.read, self.write, self.execute, self.octal_num
        )
    }
}

impl fmt::Display for ArgumentErrorLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Wrong number of arguments: {}, expected number: {}\n",
            self.length, self.expected
        )
    }
}

impl Error for ArgumentErrorLength {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

//------------------------------------------------------------------------------
// Permission values associated methods
//------------------------------------------------------------------------------

impl PermValues {
    // Get permission from user input
    fn from_user_input(args: Vec<String>) -> Result<PermValues, ArgumentErrorLength> {
        let mut perm_values = PermValues {
            read: false,
            write: false,
            execute: false,
            octal_num: 0,
        };
        if args.is_empty() {
            return Ok(perm_values);
        }

        if args.contains(&"r".to_string()) {
            perm_values.read = true;
            perm_values.octal_num = 4;
        }

        if args.contains(&"w".to_string()) {
            perm_values.write = true;
            perm_values.octal_num = perm_values.octal_num + 2;
        }

        if args.contains(&"e".to_string()) {
            perm_values.execute = true;
            perm_values.octal_num = perm_values.octal_num + 1;
        }

        // Inform user about invalid argument
        for arg in args {
            if arg != "r".to_string() && arg != "w".to_string() && arg != "e".to_string() {
                eprintln!("Ignoring invalid argument: {}", &arg);
            }
        }

        return Ok(perm_values);
    }

    // Get permission as row
    fn as_row(&self, header: &str) -> Row {
        return row![header, " ", " ", self.read, self.write, self.execute];
    }

    // Get permission as raw
    fn as_raw(&self) -> String {
        let mut value = String::from("");
        if self.read {
            value.push_str(&"r".to_string());
        } else {
            value.push_str(&"-".to_string());
        }

        if self.write {
            value.push_str(&"w".to_string());
        } else {
            value.push_str(&"-".to_string());
        }

        if self.execute {
            value.push_str(&"x".to_string());
        } else {
            value.push_str(&"-".to_string());
        }

        return value;
    }
}

//------------------------------------------------------------------------------
// handle print subcmd
//------------------------------------------------------------------------------

pub fn handle_print(matches: &ArgMatches) -> Result<bool, failure::Error> {
    let file_name = matches.value_of("file_name").unwrap().to_string();

    let fperm = FilePermission::from_name(&file_name)?;

    match matches.subcommand() {
        ("simple", Some(m)) => simple_style_subcmd(m, fperm),
        ("unix", Some(m)) => unix_style_subcmd(m, fperm),
        ("number", Some(_m)) => number_representation_subcmd(fperm),
        _ => fperm.print(PrintStyle::Simple),
    }

    return Ok(true);
}

//------------------------------------------------------------------------------
// Simple style print
//------------------------------------------------------------------------------

fn simple_style_subcmd(arg: &clap::ArgMatches, fperm: FilePermission) {
    let mut perm_table = Table::new();
    perm_table.set_titles(row![
        "File Permissions",
        "Is directory",
        "Is symlink",
        "read",
        "write",
        "execute"
    ]);

    perm_table.add_row(row![fperm.name, fperm.dir]);
    perm_table.add_row(row!["", "", fperm.symlink]);

    if arg.is_present("user") {
        perm_table.add_row(fperm.user.as_row("User"));
    }

    if arg.is_present("group") {
        perm_table.add_row(fperm.group.as_row("Group"));
    }

    if arg.is_present("other") {
        perm_table.add_row(fperm.other.as_row("Other"));
    }

    if perm_table.len() == 2 {
        perm_table.add_row(fperm.user.as_row("User"));
        perm_table.add_row(fperm.group.as_row("Group"));
        perm_table.add_row(fperm.other.as_row("Other"));
    }

    perm_table.printstd();
}

//------------------------------------------------------------------------------
// unix style print
//------------------------------------------------------------------------------

fn unix_style_subcmd(arg: &clap::ArgMatches, fperm: FilePermission) {
    let mut output_string = String::from("");

    let p1 = if fperm.symlink {
        "l".to_string()
    } else {
        if fperm.dir {
            "d".to_string()
        } else {
            ".".to_string()
        }
    };

    output_string.push_str(&p1);

    if arg.is_present("user") {
        output_string.push_str(&fperm.user.as_raw());
    }

    if arg.is_present("group") {
        output_string.push_str(&fperm.group.as_raw());
    }

    if arg.is_present("other") {
        output_string.push_str(&fperm.other.as_raw());
    }

    if output_string.len() == 1 {
        output_string.push_str(&fperm.user.as_raw());
        output_string.push_str(&fperm.group.as_raw());
        output_string.push_str(&fperm.other.as_raw());
    }

    output_string.push_str(&"   ".to_string());
    println!("{}", output_string);

    //unimplemented!();
}

//------------------------------------------------------------------------------
// Number representation print
//------------------------------------------------------------------------------

fn number_representation_subcmd(fperm: FilePermission) {
    println!(
        "Permission number representation of '{}' is: {}{}{}{}",
        fperm.name,
        fperm.dir as u32,
        fperm.user.octal_num,
        fperm.group.octal_num,
        fperm.other.octal_num
    );
}

//------------------------------------------------------------------------------
// Calculate number representation subcmd
//------------------------------------------------------------------------------
pub fn handle_calculate(matches: &ArgMatches) -> Result<bool, failure::Error> {
    let mut user: Vec<String> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    let mut other: Vec<String> = Vec::new();

    // Get user permissions
    if matches.is_present("user") {
        let args_string = matches.value_of("user").unwrap();
        let user_args: Vec<_> = args_string.chars().collect();
        for arg in user_args {
            user.push(arg.to_string());
        }
    }

    // Get group permissions
    if matches.is_present("group") {
        let args_string = matches.value_of("group").unwrap();
        let group_args: Vec<_> = args_string.chars().collect();
        for arg in group_args {
            group.push(arg.to_string());
        }
    }

    // Get other permissions
    if matches.is_present("other") {
        let args_string = matches.value_of("other").unwrap();
        let other_args: Vec<_> = args_string.chars().collect();
        for arg in other_args {
            other.push(arg.to_string());
        }
    }

    // Set permission from input arguments
    let user_perm = PermValues::from_user_input(user)?;
    let group_perm = PermValues::from_user_input(group)?;
    let other_perm = PermValues::from_user_input(other)?;

    // Print output
    println!(
        "Permission number representation: {}{}{}",
        user_perm.octal_num, group_perm.octal_num, other_perm.octal_num
    );

    return Ok(true);
}
