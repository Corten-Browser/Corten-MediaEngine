//! Unit tests for Matroska demuxer

use cortenbrowser_format_parsers::{Demuxer, MatroskaDemuxer};

/// Test that MatroskaDemuxer can be created
#[test]
fn test_matroska_demuxer_new() {
    let _demuxer = MatroskaDemuxer::new();
    // Should not panic
}

/// Test parsing invalid Matroska data returns error
#[test]
fn test_matroska_demuxer_parse_invalid_data() {
    let demuxer = MatroskaDemuxer::new();
    let invalid_data = b"not a Matroska file";

    let result = demuxer.parse(invalid_data);
    assert!(result.is_err(), "Should fail to parse invalid data");
}

/// Test parsing empty data returns error
#[test]
fn test_matroska_demuxer_parse_empty_data() {
    let demuxer = MatroskaDemuxer::new();
    let empty_data = b"";

    let result = demuxer.parse(empty_data);
    assert!(result.is_err(), "Should fail to parse empty data");
}
