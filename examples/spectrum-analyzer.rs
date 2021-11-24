use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use grounded::complex::Complex;
use grounded::fft::fft;
use minifb::{Key, Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use ringbuf::RingBuffer;
use std::collections::VecDeque;
use std::error::Error;

const BANDWIDTH: usize = 8192;

fn main() -> Result<(), Box<dyn Error>> {
    let mut size = (640, 360);
    let mut window = Window::new(
        "Spectrum Analyzer",
        size.0,
        size.1,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )?;

    let mut buffer: Vec<u32> = vec![0; size.0 * size.1];

    let (mut audio_tx, mut audio_rx) = RingBuffer::<Complex>::new(BANDWIDTH).split();
    let mut convert_buffer = vec![Complex::ZERO; BANDWIDTH];

    let audio_host = cpal::default_host();
    let audio_device = audio_host
        .default_input_device()
        .ok_or("cannot find default input device")?;
    let input_config = audio_device.default_input_config()?;
    let sample_rate = input_config.sample_rate().0 as f64;
    let stream = audio_device.build_input_stream(
        &input_config.into(),
        move |data: &[f32], _| {
            let len = data.len().min(BANDWIDTH);
            let slice = &data[data.len() - len..];
            for (sample, slot) in slice.iter().zip(&mut convert_buffer) {
                *slot = Complex::new(*sample as f64, 0.0);
            }
            audio_tx.push_slice(&convert_buffer[..len]);
        },
        move |err| {
            eprintln!("{:?}", err);
        },
    )?;
    stream.play()?;
    let mut fft_deque = VecDeque::new();
    fft_deque.resize(BANDWIDTH, Complex::ZERO);
    let mut magnitudes = vec![(0.0_f64, 0.0_f64); BANDWIDTH / 2];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = window.get_size();
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
        }

        let mut consumed = 0;
        audio_rx.access(|a, b| {
            consumed = a.len() + b.len();
            drop(fft_deque.drain(..consumed));
            fft_deque.extend(a);
            fft_deque.extend(b);
        });
        audio_rx.discard(consumed);
        let mut fft_buffer = fft_deque.make_contiguous().to_vec();
        let damping = 2.0;
        for (i, sample) in fft_buffer.iter_mut().rev().enumerate() {
            *sample *= 10.0_f64.powf(i as f64 * -damping / (BANDWIDTH as f64));
        }
        fft(&mut fft_buffer);

        for i in 0..BANDWIDTH / 2 {
            let z = fft_buffer[i];
            magnitudes[i] = (
                i as f64 * sample_rate / (BANDWIDTH as f64),
                ((z.re * z.re + z.im * z.im) / (BANDWIDTH as f64)).sqrt(),
            );
        }

        {
            let backend = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                bytemuck::cast_slice_mut(&mut buffer),
                (size.0 as u32, size.1 as u32),
            )?;

            let drawing_area = backend.into_drawing_area();

            drawing_area.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&drawing_area)
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(40)
                .build_cartesian_2d((10.0..sample_rate / 2.0).log_scale(), 0.0..1.0)?;
            chart
                .configure_mesh()
                .x_desc("Frequency (Hz)")
                .y_desc("Some kind of magnitude idk")
                .draw()?;
            chart.draw_series(LineSeries::new(magnitudes.iter().copied(), RED))?;

            drawing_area.present()?;
        }

        window.update_with_buffer(&buffer, size.0, size.1)?;
    }
    Ok(())
}
