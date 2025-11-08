
use clap::{Parser, Subcommand};
use motogarage_parser::{Garage, parse_moto_file};
use std::fs; 
use std::path::PathBuf; 

// --- 1. COMMAND-LINE INTERFACE  DEFINITION ---
// 'clap' uses this struct to generate the --help menu and parse args.
#[derive(Parser, Debug)]
#[command(
  name = "moto",
  version = "1.0",
  about = "A parser and interpreter for MotoGarage DSL" 
)]
struct Cli {
    // This holds whichever subcommand the user chose (e.g., 'parse' or 'credits').
  #[command(subcommand)]
  command: Commands,
}

// Defines the available subcommands.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses and executes a .moto file
  Parse {
        /// The path to the .moto file
    #[arg(required = true)]
    file_path: PathBuf,
  },
    /// Displays author information
  Credits,
}

// --- 2. MAIN FUNCTION  ---
// The main entry point for the executable.
// It returns 'anyhow::Result<()>' for easy, top-level error handling.
fn main() -> anyhow::Result<()> {
    // 'clap' parses arguments from the command line.
  let cli = Cli::parse();

    // Figure out which command the user ran.
  match cli.command {
        // --- 'parse' COMMAND LOGIC ---
    Commands::Parse { file_path } => {
            // Use 'eprintln!' for status messages (logs).
            // This goes to 'stderr', separating it from the actual program output.
      eprintln!("[INFO] Reading file: {:?}", file_path);

            // 1. Read the file content into a string.
      let content = fs::read_to_string(&file_path)
                // 'anyhow' lets us easily add context to errors.
        .map_err(|e| anyhow::anyhow!("Cannot read file {:?}: {}", file_path, e))?;
            
            // 2. Call our library to parse the file content into an AST.
            // The '?' operator will automatically convert our library's 'MotoError'
            // into an 'anyhow::Error' and return it if something fails.
      let ast = parse_moto_file(&content)?;
      eprintln!("[INFO] File parsed.");

      eprintln!("[INFO] Procesing queries...");
            // 3. Create the interpreter and execute the AST.
      let mut garage = Garage::new();
      let results = garage.execute(ast)?; // '?' also handles interpreter errors.

            // 4. Print the results.
      if results.is_empty() {
        eprintln!("No result from queries."); // Log message to stderr
      } else {
                // Use 'println!' for the actual, successful output.
                // This goes to 'stdout', so it can be piped to other programs.
        println!("--- Result ---");
        for result in results {
          println!("- {}", result);
        }
      }
    }

        // --- 'credits' COMMAND LOGIC ---
    Commands::Credits => {
      println!("--- Credits ---");
      println!("MotoGarage DSL Parser v1.0");
      println!("Author: Shakh Yaroslav");
    }
  }

  Ok(()) // Everything finished successfully.
}