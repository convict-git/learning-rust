use std::{fs::File, io::Error, io::ErrorKind, io::Read};

pub fn propagating_error_basic(file_path: &str) -> Result<String, Error> {
    // read from file
    let file = File::open(file_path);
    let mut resolved_file = match file {
        Ok(f) => f,
        Err(e) => {
            // If file not found, add more detail in the error
            // TODO: later figure out how to enhance this error object itself
            if e.kind() == ErrorKind::NotFound {
                println!("{file_path} not found. Kindly make sure it's relative to the root directory of the crate");
            }
            return Err(e); // NOTE: here return type of this arm is () unit
        } // but it can early return from the function with Err(e)
    };

    // convert to string
    let mut file_content = String::new();
    // NOTE: you won't find read_to_string on File unless you import the trait std::io::Read
    return match resolved_file.read_to_string(&mut file_content) {
        Ok(_) => Ok(file_content),
        Err(e) => Err(e),
    };
}

pub fn propagating_errors(file_path: &str) -> Result<String, Error> {
    // ? can be used whichever type implements FromResidual like Option or Result

    let mut file_content = String::new();

    let mut resolved_file = File::open(file_path)?;
    resolved_file.read_to_string(&mut file_content)?;

    /*
    // OR -- shorter
    File::open(file_path)?.read_to_string(&mut file_content)?;

    // OR -- inline read_to_string
    {
        let this = &mut File::open(file_path)?;
        let buf: &mut String = &mut file_content;
        this.read_to_string(buf)
    }?;

    // OR -- inline open
    std::fs::OpenOptions::new()
        .read(true)
        .open(file_path)?
        .read_to_string(&mut file_content)?;
    */

    return Ok(file_content);
}

struct OtherError {
    message: String,
}

// NOTE: You can have Result with various E types (and Options)
// Then, how do you return a single type of Error in such cases? `From` trait helps here
// impl From<Ec> for E {} -> then you can tell the compiler how to turn error of type E to Ec

// NOTE: Also, The main function can return any type that implements std::process::Termination
// trait
enum ReturnedErrors {
    X,
    Y,
}
struct ReturnType(ReturnedErrors);
impl std::process::Termination for ReturnType {
    fn report(self) -> std::process::ExitCode {
        match self.0 {
            ReturnedErrors::X => 1.into(),
            ReturnedErrors::Y => 2.into(),
        }
    }
}

pub fn propagating_errors_with_option(file_path: &str) -> Option<String> {
    let mut file_content = String::new();
    // File::open(file_path)?.read_to_string(&mut file_content)?; /* ERROR - NOTE: ? operator can NOT be used interchangibly for Result and Option here, File::open and read_to_string return Result but the function returns Option */
    File::open(file_path)
        // Convert the Result into Option
        .ok()?
        .read_to_string(&mut file_content)
        .ok()?;

    Some(file_content)
}

// Some good pointers
// -- Return a Result when error is "expected"
// -- panic! when contract is breached / the calling code cannot be recovered
// Instead of relying a lot on error checking, we should handle that job to the compiler
// by introducing strong typing
