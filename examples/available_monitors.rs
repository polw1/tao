// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

use tao::{event_loop::EventLoop, monitor::MonitorHandle};

fn main() {
  env_logger::init();

  let event_loop = EventLoop::new();
  let primary_monitor = event_loop.primary_monitor();
  let monitors = event_loop.available_monitors().collect::<Vec<_>>();

  println!("Available monitors: {}", monitors.len());

  if monitors.is_empty() {
    println!("No monitors found.");
    return;
  }

  for (index, monitor) in monitors.iter().enumerate() {
    print_monitor(index, monitor, primary_monitor.as_ref());
  }
}

fn print_monitor(index: usize, monitor: &MonitorHandle, primary_monitor: Option<&MonitorHandle>) {
  let position = monitor.position();
  let size = monitor.size();
  let video_modes = monitor.video_modes().collect::<Vec<_>>();
  let is_primary = primary_monitor.is_some_and(|primary| primary == monitor);

  println!();
  println!(
    "Monitor #{index}{}",
    if is_primary { " (primary)" } else { "" }
  );
  println!(
    "  name: {}",
    monitor.name().unwrap_or_else(|| "<unknown>".into())
  );
  println!("  position: x={}, y={}", position.x, position.y);
  println!("  resolution: {}x{}", size.width, size.height);
  println!("  scale factor: {}", monitor.scale_factor());
  println!("  debug: {monitor:#?}");

  if video_modes.is_empty() {
    println!("  video modes: <none reported>");
  } else {
    println!("  video modes:");
    for (mode_index, mode) in video_modes.iter().enumerate() {
      let size = mode.size();

      println!(
        "    #{mode_index}: {}x{} @ {} Hz ({} bpp)",
        size.width,
        size.height,
        mode.refresh_rate(),
        mode.bit_depth()
      );
    }
  }
}
