use grbli::device::response::compile_option::*;


#[test]
fn from_parses_message_correctly() {
    let message_str = "[OPT:$2L,10,255]";
    let compile_option = CompileOptionsResponse::from(message_str).unwrap();
    assert_eq!(10, compile_option.block_buffer_size());
    assert_eq!(255, compile_option.rx_buffer_size());

    assert_eq!(3, compile_option.options().len());
    assert_eq!(0, compile_option.unknown_options().len());
    assert!(matches!(compile_option.options()[0], CompileOption::RestoreEEPROMDollarSettingsDisabled));
    assert!(matches!(compile_option.options()[1], CompileOption::DualAxisMotorsWithSelfSquaringEnabled));
    assert!(matches!(compile_option.options()[2], CompileOption::AlarmStateOnPowerUpWhenHomingInitLock));
}

#[test]
fn from_parses_unknown_options_correctly() {
    let message_str = "[OPT:$2Lc,10,255]";
    let compile_option = CompileOptionsResponse::from(message_str).unwrap();
    assert_eq!(10, compile_option.block_buffer_size());
    assert_eq!(255, compile_option.rx_buffer_size());

    assert_eq!(3, compile_option.options().len());
    assert!(matches!(compile_option.options()[0], CompileOption::RestoreEEPROMDollarSettingsDisabled));
    assert!(matches!(compile_option.options()[1], CompileOption::DualAxisMotorsWithSelfSquaringEnabled));
    assert!(matches!(compile_option.options()[2], CompileOption::AlarmStateOnPowerUpWhenHomingInitLock));

    assert_eq!(1, compile_option.unknown_options().len());
    assert_eq!("c", compile_option.unknown_options()[0])
}

#[test]
fn from_accepts_an_empty_options_section() {
    let message_str = "[OPT:,10,255]";
    let compile_option = CompileOptionsResponse::from(message_str).unwrap();
    assert_eq!(10, compile_option.block_buffer_size());
    assert_eq!(255, compile_option.rx_buffer_size());

    assert_eq!(0, compile_option.options().len());
    assert_eq!(0, compile_option.unknown_options().len())
}

#[test]
fn from_cannot_read_empty_message() {
    let message_str = "[OPT:]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid compile options string \"\"", &message_error[..])
}

#[test]
fn from_cannot_read_non_numeric_block_size() {
    let message_str = "[OPT:,a,3]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid block buffer size \"a\"", &message_error[..])
}

#[test]
fn from_cannot_read_non_numeric_rx_size() {
    let message_str = "[OPT:,10,a]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid rx buffer size \"a\"", &message_error[..])
}

#[test]
fn from_cannot_read_less_than_three_segements() {
    let message_str = "[OPT:10,3]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid compile options string \"10,3\"", &message_error[..])
}

#[test]
fn from_cannot_read_more_than_three_segements() {
    let message_str = "[OPT:,10,12,3]";
    let message_error = CompileOptionsResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid compile options string \",10,12,3\"", &message_error[..])
}

/*
#[test]
fn from_applys_trimming() {
    let message_str = "  [VER:0.1223d.234f:test]      ";
    let message = VersionResponse::from(message_str).unwrap();
    assert_eq!(String::from("0.1223d.234f"), *message.get_version());
    assert_eq!(String::from("test"), *message.get_name())
}

#[test]
fn from_accepts_empty_name_section() {
    let message_str = "[VER:0.1223d.234f:]";
    let message = VersionResponse::from(message_str).unwrap();
    assert_eq!(String::from("0.1223d.234f"), *message.get_version());
    assert_eq!(String::from(""), *message.get_name())
}

#[test]
fn from_accepts_colons_in_name() {
    let message_str = "[VER:0.1223d.234f:a:b:c:d:]";
    let message = VersionResponse::from(message_str).unwrap();
    assert_eq!(String::from("0.1223d.234f"), *message.get_version());
    assert_eq!(String::from("a:b:c:d:"), *message.get_name())
}

#[test]
fn from_cannot_read_empty_messages() {
    let message_str = "[VER:]";
    let message_error = VersionResponse::from(message_str).err().unwrap();
    assert_eq!("Invalid count of version strings: \"\"", &message_error[..])
}

#[test]
fn from_fails_on_missing_prefix() {
    let message_str = "0.1223d.234f:test]";
    let message = VersionResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Could not read version: 0.1223d.234f:test]", &message_error[..])
}

#[test]
fn from_fails_on_missing_suffix() {
    let message_str = "[VER:0.1223d.234f:test";
    let message = VersionResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Could not read version: [VER:0.1223d.234f:test", &message_error[..])
}

#[test]
fn from_fails_on_missing_name_separator() {
    let message_str = "[VER:0.1223d.234f]";
    let message = VersionResponse::from(message_str);
    assert!(message.is_err());
    let message_error = message.err().unwrap();
    assert_eq!("Invalid count of version strings: \"0.1223d.234f\"", &message_error[..])
}
*/