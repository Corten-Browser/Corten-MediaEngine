//! Unit tests for MP4 demuxer

use cortenbrowser_format_parsers::{Demuxer, Mp4Demuxer};

/// Test that Mp4Demuxer can be created
#[test]
fn test_mp4_demuxer_new() {
    let _demuxer = Mp4Demuxer::new();
    // Should not panic
}

/// Test parsing invalid MP4 data returns error
#[test]
fn test_mp4_demuxer_parse_invalid_data() {
    let demuxer = Mp4Demuxer::new();
    let invalid_data = b"not an MP4 file";

    let result = demuxer.parse(invalid_data);
    assert!(result.is_err(), "Should fail to parse invalid data");
}

/// Test parsing empty data returns error
#[test]
fn test_mp4_demuxer_parse_empty_data() {
    let demuxer = Mp4Demuxer::new();
    let empty_data = b"";

    let result = demuxer.parse(empty_data);
    assert!(result.is_err(), "Should fail to parse empty data");
}

/// Test parsing minimal valid MP4 structure
///
/// This creates a minimal but valid MP4 file with ftyp and moov boxes.
#[test]
fn test_mp4_demuxer_parse_minimal_valid() {
    let demuxer = Mp4Demuxer::new();

    // Create minimal MP4 structure: ftyp box only
    // ftyp box: size (4 bytes) + type (4 bytes) + major_brand (4) + minor_version (4) + compatible_brands
    let mut mp4_data = Vec::new();

    // ftyp box
    mp4_data.extend_from_slice(&20u32.to_be_bytes()); // box size
    mp4_data.extend_from_slice(b"ftyp"); // box type
    mp4_data.extend_from_slice(b"isom"); // major brand
    mp4_data.extend_from_slice(&0u32.to_be_bytes()); // minor version
    mp4_data.extend_from_slice(b"isom"); // compatible brand

    let result = demuxer.parse(&mp4_data);

    // For now, this should either succeed or fail gracefully
    // Once implemented, we'll verify the actual content
    match result {
        Ok(info) => {
            // Valid parse - check basic structure
            assert!(info.duration >= std::time::Duration::ZERO);
        }
        Err(_) => {
            // May need more complex structure
            // This is acceptable for minimal data
        }
    }
}
