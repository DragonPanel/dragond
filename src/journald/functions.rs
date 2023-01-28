use std::process::Command;
use std::string::String;


/// Reads specified amount of latest log lines for specific unit from journal and return it as string with json formatting
///
/// Takes a unit name and a desired number of lines. The '.service' suffix can be omitted
/// Return exit code in case of error.
///
pub fn read_n_latest_lines(unit_name: &str, lines_num: &usize) -> Result<String, i32> {
  let output = Command::new("journalctl")
    .arg("--no-pager")
    .arg("-r")
    .args(["-o", "json"])
    .args(["-u", &unit_name])
    .args(["-n", &lines_num.to_string()])
    .output();

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
    if character.eq_ignore_ascii_case(&10)  {
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

