use std::process::Command;
use std::string::String;

/// Reads journal entries for specified unit. The .service suffix can be omitted.
///
/// If lines_num is provided, reads only that amount of entries.
/// If cursor is provided, reads entries since that cursor.
///
/// In case of error when executing command, return exit code.
///
pub fn read_lines(
  unit_name: &String,
  lines_num: &Option<usize>,
  cursor: &Option<String>,
) -> Result<String, i32> {
  let mut command = Command::new("journalctl");
  command.arg("--no-pager");
  command.arg("--reverse");
  command.args(["--output", "json"]);
  command.args(["--unit", &unit_name]);

  if cursor.is_some() {
    command.args(["--cursor", cursor.as_ref().unwrap()]);
  }

  if lines_num.is_some() {
    command.args(["--lines", &lines_num.unwrap().to_string()]);
  }

  let output = command.output();

  //TODO in case of error, returning both status code and error msg
  if output.is_err() {
    let exit_code = match output.unwrap().status.code() {
      Some(code) => code,
      None => -1,
    };

    return Err(exit_code);
  }

  // replaces newlines with commas and adds square brackets to end and beginning
  let mut command_stdout = output.unwrap().stdout;
  for character in &mut command_stdout {
    if character.eq_ignore_ascii_case(&10) {
      *character = 44;
    }
  }
  command_stdout.push(93);
  command_stdout.reverse();
  command_stdout.push(91);
  command_stdout.reverse();

  let command_output = String::from_utf8_lossy(&command_stdout);

  Ok(command_output.to_string())
}
