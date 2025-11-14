//! Unit tests for Ogg demuxer

use cortenbrowser_format_parsers::{Demuxer, OggDemuxer};

/// Test that OggDemuxer can be created
#[test]
fn test_ogg_demuxer_new() {
    let _demuxer = OggDemuxer::new();
    // Should not panic
}

/// Test parsing invalid Ogg data returns error
#[test]
fn test_ogg_demuxer_parse_invalid_data() {
    let demuxer = OggDemuxer::new();
    let invalid_data = b"not an Ogg file";

    let result = demuxer.parse(invalid_data);
    assert!(result.is_err(), "Should fail to parse invalid data");
}

/// Test parsing empty data returns error
#[test]
fn test_ogg_demuxer_parse_empty_data() {
    let demuxer = OggDemuxer::new();
    let empty_data = b"";

    let result = demuxer.parse(empty_data);
    assert!(result.is_err(), "Should fail to parse empty data");
}
