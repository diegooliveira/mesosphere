
pub mod deploy;
pub mod status;
pub mod list;
pub mod upload;

use help::HelpPrinter;
use arguments::Arguments;

/// The command structure
pub trait Command {
	fn show_short_help(&self, &mut HelpPrinter);
	fn show_long_help(&self, &mut HelpPrinter);

	fn execute(&self, mut args: Arguments);
	fn is_called(&self, &String) -> bool;
}
