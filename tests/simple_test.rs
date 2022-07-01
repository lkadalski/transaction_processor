use assert_cmd::Command;
use predicates::prelude::*;
#[test]
pub fn invalid_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("transaction_processor")?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure();
    Ok(())
}

#[test]
pub fn correct_start() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("transaction_processor")?;
    cmd.arg("test_data/01.csv");
    cmd.assert().success();
    Ok(())
}

#[test]
pub fn correctly_formatted_output() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("transaction_processor")?
        .arg("test_data/02.csv")
        .assert()
        .stdout(predicate::str::starts_with(
            "client,available,held,total,locked\n1,0.0000,0.0000,0.0000,false\n2,308.8503,682.2051,991.0554,false\n3,0.0000,682.2051,682.2051,false\n",
        ));

    Ok(())
}
#[test]
pub fn advance_case_test() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("transaction_processor")?
        .arg("test_data/03.csv")
        .assert()
        .stdout(predicate::str::starts_with(
            "client,available,held,total,locked\n1,0.0000,0.0000,0.0000,false\n2,308.8503,0.0000,308.8503,false\n3,0.0000,0.0000,0.0000,true\n",
        ));

    Ok(())
}
