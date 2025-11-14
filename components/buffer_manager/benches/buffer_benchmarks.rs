use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cortenbrowser_buffer_manager::{RingBuffer, FrameCache, BufferManager, BufferConfig};
use cortenbrowser_shared_types::{VideoFrame, PixelFormat, FrameMetadata};
use std::time::Duration;

fn ring_buffer_write_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ring_buffer_write");

    for size in [1024, 4096, 16384, 65536] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut buffer = RingBuffer::new(size * 2);
            let data = vec![0u8; size];

            b.iter(|| {
                buffer.write(black_box(&data)).unwrap()
            });
        });
    }

    group.finish();
}

fn ring_buffer_read_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ring_buffer_read");

    for size in [1024, 4096, 16384, 65536] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut buffer = RingBuffer::new(size * 2);
            let data = vec![0u8; size];
            buffer.write(&data).unwrap();

            let mut out = vec![0u8; size];

            b.iter(|| {
                let mut buf = RingBuffer::new(size * 2);
                buf.write(&data).unwrap();
                buf.read(black_box(&mut out)).unwrap()
            });
        });
    }

    group.finish();
}

fn frame_cache_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame_cache_insert");

    for frame_size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::from_parameter(frame_size), &frame_size, |b, &frame_size| {
            let mut cache = FrameCache::new(100);
            let frame = VideoFrame {
                width: 1920,
                height: 1080,
                format: PixelFormat::YUV420,
                data: vec![0u8; frame_size],
                timestamp: Duration::from_secs(0),
                duration: Some(Duration::from_millis(33)),
                metadata: FrameMetadata::default(),
            };

            b.iter(|| {
                cache.insert(black_box(frame.clone())).unwrap()
            });
        });
    }

    group.finish();
}

fn frame_cache_get_benchmark(c: &mut Criterion) {
    let mut cache = FrameCache::new(100);

    for i in 0..50 {
        let frame = VideoFrame {
            width: 1920,
            height: 1080,
            format: PixelFormat::YUV420,
            data: vec![0u8; 1000],
            timestamp: Duration::from_secs(i),
            duration: Some(Duration::from_millis(33)),
            metadata: FrameMetadata::default(),
        };
        cache.insert(frame).unwrap();
    }

    c.bench_function("frame_cache_get", |b| {
        b.iter(|| {
            cache.get(black_box(Duration::from_secs(25)))
        });
    });
}

fn buffer_manager_allocate_benchmark(c: &mut Criterion) {
    c.bench_function("buffer_manager_allocate_video", |b| {
        let config = BufferConfig::default();

        b.iter(|| {
            let mut manager = BufferManager::new(config.clone());
            manager.allocate_video_buffer(black_box(1920 * 1080)).unwrap()
        });
    });

    c.bench_function("buffer_manager_allocate_audio", |b| {
        let config = BufferConfig::default();

        b.iter(|| {
            let mut manager = BufferManager::new(config.clone());
            manager.allocate_audio_buffer(black_box(48000)).unwrap()
        });
    });
}

criterion_group!(
    benches,
    ring_buffer_write_benchmark,
    ring_buffer_read_benchmark,
    frame_cache_insert_benchmark,
    frame_cache_get_benchmark,
    buffer_manager_allocate_benchmark
);
criterion_main!(benches);
