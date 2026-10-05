#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "🧪 Running Elevated Save & Path Normalization Test Suite"
echo "=========================================================="

TEMP_DIR=$(mktemp -d)
trap 'rm -rf "${TEMP_DIR}"' EXIT

TEST_SRC="${TEMP_DIR}/src.txt"
TEST_DST="${TEMP_DIR}/dst.txt"
echo "test elevated save payload" > "${TEST_SRC}"

# 1. Compile the execute_file_write logic with Rust
cat << 'RUST_EOF' > "${TEMP_DIR}/test_runner.rs"
use std::io::Write as _;
use std::path::{Path, PathBuf};

fn execute_file_write(source: &Path, target: &Path) -> Result<(), String> {
    let source_str = source.to_string_lossy();
    let source_trimmed = source_str.trim_end_matches(['/', '\\']);
    let source_buf;
    let source: &Path = if source_trimmed.is_empty() {
        source
    } else {
        source_buf = PathBuf::from(source_trimmed);
        &source_buf
    };

    let target_str = target.to_string_lossy();
    let target_trimmed = target_str.trim_end_matches(['/', '\\']);
    let target_buf;
    let target: &Path = if target_trimmed.is_empty() {
        target
    } else {
        target_buf = PathBuf::from(target_trimmed);
        &target_buf
    };

    if target.is_dir() {
        return Err(format!("Target path exists and is a directory: {}", target.display()));
    }
    if !source.is_file() {
        return Err(format!("Source path must be an existing file: {}", source.display()));
    }
    if !source.is_absolute() || !target.is_absolute() {
        return Err("Both source and target must be absolute paths".to_string());
    }
    if source == target {
        return Err("Source and target must not be the same path".to_string());
    }
    if target.exists() && !target.is_file() {
        return Err(format!("Target path exists but is not a regular file: {}", target.display()));
    }

    #[cfg(unix)]
    let orig_mode = {
        use std::os::unix::fs::PermissionsExt as _;
        target.metadata().ok().map(|m| m.permissions().mode())
    };

    let data = std::fs::read(source).map_err(|e| format!("read error: {}", e))?;
    let mut file = std::fs::File::create(target).map_err(|e| format!("create error: {}", e))?;
    file.write_all(&data).map_err(|e| format!("write error: {}", e))?;
    file.sync_all().map_err(|e| format!("sync error: {}", e))?;

    #[cfg(unix)]
    if let Some(mode) = orig_mode {
        use std::os::unix::fs::PermissionsExt as _;
        let _ = std::fs::set_permissions(target, std::fs::Permissions::from_mode(mode));
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <source> <target>", args[0]);
        std::process::exit(1);
    }
    let src = Path::new(&args[1]);
    let dst = Path::new(&args[2]);
    match execute_file_write(src, dst) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}
RUST_EOF

rustc -O "${TEMP_DIR}/test_runner.rs" -o "${TEMP_DIR}/file_writer"

echo "✅ Compiled test runner binary successfully."

# Test 1: Normal file write
echo -n "Test 1 (Standard file write): "
"${TEMP_DIR}/file_writer" "${TEST_SRC}" "${TEST_DST}"
if [[ "$(< "${TEST_DST}")" == "test elevated save payload" ]]; then
    echo "PASSED"
else
    echo "FAILED"
    exit 1
fi

# Test 2: Target with trailing slash (the exact bug that caused 'Is a directory (os error 21)')
echo -n "Test 2 (Target path with trailing slash): "
"${TEMP_DIR}/file_writer" "${TEST_SRC}" "${TEST_DST}/"
if [[ "$(< "${TEST_DST}")" == "test elevated save payload" ]]; then
    echo "PASSED"
else
    echo "FAILED"
    exit 1
fi

# Test 3: Multiple trailing slashes
echo -n "Test 3 (Multiple trailing slashes): "
"${TEMP_DIR}/file_writer" "${TEST_SRC}" "${TEST_DST}///"
if [[ "$(< "${TEST_DST}")" == "test elevated save payload" ]]; then
    echo "PASSED"
else
    echo "FAILED"
    exit 1
fi

# Test 4: Target is an existing directory - should reject with clear message, NOT crash or return raw OS error 21
echo -n "Test 4 (Target is an actual directory): "
mkdir -p "${TEMP_DIR}/real_directory"
set +e
ERR_OUT=$("${TEMP_DIR}/file_writer" "${TEST_SRC}" "${TEMP_DIR}/real_directory" 2>&1)
EXIT_CODE=$?
set -e
if [[ ${EXIT_CODE} -ne 0 && "${ERR_OUT}" =~ "is a directory" ]]; then
    echo "PASSED (Cleanly rejected with: ${ERR_OUT})"
else
    echo "FAILED (Unexpected output or exit code: ${ERR_OUT})"
    exit 1
fi

# Test 5: Verify permissions preservation
echo -n "Test 5 (Permission preservation): "
chmod 0600 "${TEST_DST}"
"${TEMP_DIR}/file_writer" "${TEST_SRC}" "${TEST_DST}/"
CURRENT_PERM=$(stat -c "%a" "${TEST_DST}")
if [[ "${CURRENT_PERM}" == "600" ]]; then
    echo "PASSED (Permissions 0600 preserved)"
else
    echo "FAILED (Permissions changed to ${CURRENT_PERM})"
    exit 1
fi

echo "=========================================================="
echo "🎉 ALL ELEVATED SAVE UNIT TESTS PASSED SUCCESSFULLY!"
echo "=========================================================="
