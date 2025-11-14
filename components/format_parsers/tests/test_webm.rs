//! Unit tests for WebM demuxer

use cortenbrowser_format_parsers::{Demuxer, WebmDemuxer};

/// Test that WebmDemuxer can be created
#[test]
fn test_webm_demuxer_new() {
    let _demuxer = WebmDemuxer::new();
    // Should not panic
}

/// Test parsing invalid WebM data returns error
#[test]
fn test_webm_demuxer_parse_invalid_data() {
    let demuxer = WebmDemuxer::new();
    let invalid_data = b"not a WebM file";

    let result = demuxer.parse(invalid_data);
    assert!(result.is_err(), "Should fail to parse invalid data");
}

/// Test parsing empty data returns error
#[test]
fn test_webm_demuxer_parse_empty_data() {
    let demuxer = WebmDemuxer::new();
    let empty_data = b"";

    let result = demuxer.parse(empty_data);
    assert!(result.is_err(), "Should fail to parse empty data");
}
