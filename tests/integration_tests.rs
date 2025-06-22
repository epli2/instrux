use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::fs;
use std::process::Command; // Run programs
// use std::path::Path; // Not strictly needed as fs::read_to_string and Path::join work with &str
use tempfile::tempdir; // Create temporary directories

// Helper function to get the path to the compiled binary
fn get_binary_path() -> String {
    env!("CARGO_BIN_EXE_instrux").to_string()
}

#[test]
fn test_generate_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let instrux_dir = temp_dir.path().join(".instrux");
    fs::create_dir(&instrux_dir)?;
    fs::copy(
        "tests/fixtures/simple.yaml",
        instrux_dir.join("instrux.yaml"),
    )?;

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "[generate] codex形式の出力を生成: CODEX.md",
        ))
        .stdout(predicate::str::contains(
            "[generate] cline形式の出力を生成: .clinerules/instructions.md",
        ));

    let codex_content = fs::read_to_string(temp_dir.path().join("CODEX.md"))?;
    assert!(codex_content.contains("Test Project"));
    assert!(codex_content.contains("This is a code instruction."));

    let cline_content = fs::read_to_string(temp_dir.path().join(".clinerules/instructions.md"))?;
    assert!(cline_content.contains("Test Project"));
    assert!(cline_content.contains("This is a code instruction."));

    Ok(())
}

#[test]
fn test_generate_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let instrux_dir = temp_dir.path().join(".instrux");
    fs::create_dir(&instrux_dir)?;
    fs::copy(
        "tests/fixtures/simple.yaml",
        instrux_dir.join("instrux.yaml"),
    )?;

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("--- Codex ---"))
        .stdout(predicate::str::contains("--- Cline ---"));

    // Assert that files are not actually created
    assert!(!temp_dir.path().join("CODEX.md").exists());
    assert!(
        !temp_dir
            .path()
            .join(".clinerules/instructions.md") // Updated path
            .exists()
    );

    Ok(())
}

#[test]
fn test_generate_overwrite() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let instrux_dir = temp_dir.path().join(".instrux");
    fs::create_dir_all(&instrux_dir)?; // Create .instrux and its parents if they don't exist
    fs::copy(
        "tests/fixtures/simple.yaml",
        instrux_dir.join("instrux.yaml"),
    )?;

    // Create dummy output files
    fs::write(temp_dir.path().join("CODEX.md"), "old codex content")?;
    let clinerules_dir = temp_dir.path().join(".clinerules");
    fs::create_dir_all(&clinerules_dir)?;
    fs::write(
        clinerules_dir.join("instructions.md"), // Updated path
        "old cline content",
    )?;

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .arg("--overwrite")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CODEX.md.bak をバックアップしました",
        ))
        .stdout(predicate::str::contains("CODEX.md を上書きしました"))
        .stdout(predicate::str::contains(
            ".clinerules/instructions.md.bak をバックアップしました",
        ))
        .stdout(predicate::str::contains(
            ".clinerules/instructions.md を上書きしました",
        ));

    // Assert that backup files exist
    assert!(temp_dir.path().join("CODEX.md.bak").exists());
    assert!(
        temp_dir
            .path()
            .join(".clinerules/instructions.md.bak") // Updated path
            .exists()
    );

    // Assert that new files are created and content is correct
    let codex_content = fs::read_to_string(temp_dir.path().join("CODEX.md"))?;
    assert!(codex_content.contains("Test Project"));
    let cline_content = fs::read_to_string(
        temp_dir.path().join(".clinerules/instructions.md"), // Updated path
    )?;
    assert!(cline_content.contains("Test Project"));

    // Assert that backup content is the old content
    let codex_bak_content = fs::read_to_string(temp_dir.path().join("CODEX.md.bak"))?;
    assert_eq!(codex_bak_content, "old codex content");
    let cline_bak_content = fs::read_to_string(
        temp_dir.path().join(".clinerules/instructions.md.bak"), // Updated path
    )?;
    assert_eq!(cline_bak_content, "old cline content");

    Ok(())
}

#[test]
fn test_generate_force() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let instrux_dir = temp_dir.path().join(".instrux");
    fs::create_dir_all(&instrux_dir)?;
    fs::copy(
        "tests/fixtures/simple.yaml",
        instrux_dir.join("instrux.yaml"),
    )?;

    // Create dummy output files
    fs::write(temp_dir.path().join("CODEX.md"), "old codex content")?;
    let clinerules_dir = temp_dir.path().join(".clinerules");
    fs::create_dir_all(&clinerules_dir)?;
    fs::write(
        clinerules_dir.join("instructions.md"), // Updated path
        "old cline content",
    )?;

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("CODEX.md を強制上書きしました")) // This message seems correct
        .stdout(predicate::str::contains(
            ".clinerules/instructions.md を強制上書きしました", // Updated path
        ));

    // Assert that backup files do NOT exist
    assert!(!temp_dir.path().join("CODEX.md.bak").exists());
    assert!(
        !temp_dir
            .path()
            .join(".clinerules/instructions.md.bak") // Updated path
            .exists()
    );

    // Assert that new files are created and content is correct
    let codex_content = fs::read_to_string(temp_dir.path().join("CODEX.md"))?;
    assert!(codex_content.contains("Test Project"));
    let cline_content = fs::read_to_string(
        temp_dir.path().join(".clinerules/instructions.md"), // Updated path
    )?;
    assert!(cline_content.contains("Test Project"));

    Ok(())
}

#[test]
fn test_generate_non_existent_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    // Do not create .instrux/instrux.yaml

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .assert()
        .failure() // Expect the command to fail
        .stderr(predicate::str::contains("設定ファイルの読み込みに失敗"));

    Ok(())
}

#[test]
fn test_generate_invalid_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let instrux_dir = temp_dir.path().join(".instrux");
    fs::create_dir(&instrux_dir)?;
    fs::copy(
        "tests/fixtures/invalid.yaml", // Use the invalid config file
        instrux_dir.join("instrux.yaml"),
    )?;

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .assert()
        .failure() // Expect the command to fail
        .stderr(predicate::str::contains("設定ファイルの読み込みに失敗")); // Or a more specific error message if available

    Ok(())
}

#[test]
fn test_generate_multiple_file_base_path_is_file() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    let temp_dir = tempdir()?;
    let instrux_dir = temp_dir.path().join(".instrux");
    fs::create_dir(&instrux_dir)?;
    // outputMode: multiple のみ指定
    let config = r#"
version: '1.0.0'
instructions:
  - title: 'Test1'
    body: 'Body1'
  - title: 'Test2'
    body: 'Body2'
targets:
  cline:
    outputMode: multiple
"#;
    let config_path = instrux_dir.join("instrux.yaml");
    let mut f = std::fs::File::create(&config_path)?;
    f.write_all(config.as_bytes())?;

    let mut cmd = Command::new(get_binary_path());
    cmd.current_dir(temp_dir.path())
        .arg("generate")
        .assert()
        .success();

    // 期待される出力: .clinerules/Test1.md, .clinerules/Test2.md
    let cline1 = temp_dir.path().join(".clinerules/Test1.md");
    let cline2 = temp_dir.path().join(".clinerules/Test2.md");
    assert!(cline1.exists(), ".clinerules/Test1.md should exist");
    assert!(cline2.exists(), ".clinerules/Test2.md should exist");
    Ok(())
}
