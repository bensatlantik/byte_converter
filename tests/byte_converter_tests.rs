// tests/byte_converter_tests.rs
use byte_converter::*;

#[test]
fn test_bytes_to_kb() {
    // Basic conversion test
    assert_eq!(bytes_to_kb(2048), 2.0);

    // Edge case: 0 bytes should equal 0 KB
    assert_eq!(bytes_to_kb(0), 0.0);

    // Non-whole result test
    assert_eq!(bytes_to_kb(1536), 1.5);
}

#[test]
fn test_bytes_to_mb() {
    // Basic conversion test
    assert_eq!(bytes_to_mb(1_048_576), 1.0);

    // Edge case: 0 bytes should equal 0 MB
    assert_eq!(bytes_to_mb(0), 0.0);

    // Non-whole result test
    assert!((bytes_to_mb(1_572_864) - 1.5).abs() < f64::EPSILON); // Tolerance for floating-point comparisons
}

#[test]
fn test_kb_to_bytes() {
    // Basic conversion test
    assert_eq!(kb_to_bytes(2.0), 2048);

    // Edge case: 0 KB should equal 0 bytes
    assert_eq!(kb_to_bytes(0.0), 0);

    // Non-whole KB value test
    assert_eq!(kb_to_bytes(1.5), 1536);
}

#[test]
fn test_mb_to_bytes() {
    // Basic conversion test
    assert_eq!(mb_to_bytes(1.0), 1_048_576);

    // Edge case: 0 MB should equal 0 bytes
    assert_eq!(mb_to_bytes(0.0), 0);

    // Non-whole MB value test
    assert_eq!(mb_to_bytes(1.5), 1_572_864);
}

#[test]
fn test_kb_to_mb() {
    // Basic conversion test
    assert_eq!(kb_to_mb(2048.0), 2.0);

    // Edge case: 0 KB should equal 0 MB
    assert_eq!(kb_to_mb(0.0), 0.0);

    // Non-whole KB value test
    assert!((kb_to_mb(1536.0) - 1.5).abs() < f64::EPSILON);
}

#[test]
fn test_mb_to_kb() {
    // Basic conversion test
    assert_eq!(mb_to_kb(1.0), 1024.0);

    // Edge case: 0 MB should equal 0 KB
    assert_eq!(mb_to_kb(0.0), 0.0);

    // Non-whole MB value test
    assert_eq!(mb_to_kb(1.5), 1536.0);
}
