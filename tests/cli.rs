use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;

// Helper to locate the Rules dir at the root of the repository
fn get_rules_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("Rules");
    path
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    
    cmd.arg("--version");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_missing_rules_dir_fails() {
    let mut cmd = Command::cargo_bin("mathcat-cli").expect("Binary should build");
    
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.write_stdin("<math><mi>x</mi></math>");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: MathCAT Rules directory not found"));
}

#[test]
fn test_rules_dir_via_flag() {
    let rules = get_rules_dir();
    assert!(
        rules.exists(),
        "Missing ./Rules directory! Please run `cargo get-rules` to copy it to your project root."
    );

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    let mathml = r#"<math><mi>x</mi><mo>+</mo><mi>y</mi></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .success()
        .stdout(predicate::eq("x plus y")); 
}

#[test]
fn test_rules_dir_via_env_var() {
    let rules = get_rules_dir();
    assert!(
        rules.exists(),
        "Missing ./Rules directory! Please run `cargo get-rules` to copy it to your project root."
    );

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env("MATHCAT_RULES_DIR", rules.to_str().unwrap());
    
    let mathml = r#"<math><mi>x</mi><mo>+</mo><mi>y</mi></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .success()
        .stdout(predicate::eq("x plus y"));
}

#[test]
fn test_default_preferences() {
    let rules = get_rules_dir();
    assert!(rules.exists(), "Missing ./Rules");

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    // NO style or verbosity flags provided to test defaults (ClearSpeak + Medium)
    let mathml = r#"<math><mfrac><mn>1</mn><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .success()
        .stdout(predicate::eq("the fraction with numerator 1; and denominator x plus 1"));
}

#[test]
fn test_style_simplespeak() {
    let rules = get_rules_dir();
    assert!(rules.exists(), "Missing ./Rules");

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    cmd.arg("--style").arg("SimpleSpeak");
    cmd.arg("--verbosity").arg("Verbose");
    
    let mathml = r#"<math><mfrac><mn>1</mn><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>"#;
    cmd.write_stdin(mathml);
    
    // SimpleSpeak Verbose includes the structural indicators
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty()) 
        .stdout(predicate::eq("fraction, 1 over, x plus 1, end fraction"));
}

#[test]
fn test_style_clearspeak() {
    let rules = get_rules_dir();
    assert!(rules.exists(), "Missing ./Rules");

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    cmd.arg("--style").arg("ClearSpeak");
    cmd.arg("--verbosity").arg("Medium");
    
    let mathml = r#"<math><mfrac><mn>1</mn><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .success()
        .stdout(predicate::eq("the fraction with numerator 1; and denominator x plus 1"));
}

#[test]
fn test_verbosity_terse() {
    let rules = get_rules_dir();
    assert!(rules.exists(), "Missing ./Rules");

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    cmd.arg("--style").arg("ClearSpeak");
    cmd.arg("--verbosity").arg("Terse");
    
    // Use a nested fraction where Terse actually drops the "end fraction" structural indicators
    let mathml = r#"<math><mfrac><mn>1</mn><mrow><mi>x</mi><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow></mfrac></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty()) 
        .stdout(predicate::eq("the fraction with numerator 1; and denominator x plus 1 over y"));
}

#[test]
fn test_invalid_style_fails() {
    let rules = get_rules_dir();
    assert!(rules.exists(), "Missing ./Rules");

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    cmd.arg("--style").arg("FakeStyle");
    
    let mathml = r#"<math><mi>x</mi></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid style 'FakeStyle'"));
}

#[test]
fn test_language_spanish() {
    let rules = get_rules_dir();
    assert!(rules.exists(), "Missing ./Rules");

    let mut cmd = Command::cargo_bin("mathcat-cli").unwrap();
    cmd.env_remove("MATHCAT_RULES_DIR");
    cmd.arg("--rules-dir").arg(rules.to_str().unwrap());
    
    cmd.arg("--lang").arg("es");
    
    let mathml = r#"<math><mfrac><mn>1</mn><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>"#;
    cmd.write_stdin(mathml);
    
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::eq("la fracción con numerador 1; y denominador x más 1"));
}