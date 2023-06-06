use std::str::FromStr;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn read_input() -> Vec<String> {
  std::fs::read_to_string("benches/data.txt")
    .unwrap()
    .lines()
    .take(1000)
    .map(String::from)
    .collect::<Vec<_>>()
}

fn parse_twitch(c: &mut Criterion) {
  let input = read_input();
  c.bench_with_input(
    BenchmarkId::new("twitch", "data.txt"),
    &input,
    |b, lines| {
      b.iter(|| {
        for line in lines.clone() {
          black_box(twitch::Message::parse(line).expect("failed to parse"));
        }
      })
    },
  );
}

fn parse_twitch_irc(c: &mut Criterion) {
  let input = read_input();
  c.bench_with_input(
    BenchmarkId::new("twitch_irc", "data.txt"),
    &input,
    |b, lines| {
      b.iter(|| {
        for line in lines.clone() {
          black_box(twitch_irc::message::IRCMessage::parse(&line).expect("failed to parse"));
        }
      })
    },
  );
}

fn parse_irc_rust(c: &mut Criterion) {
  let input = read_input();
  c.bench_with_input(
    BenchmarkId::new("irc_rust", "data.txt"),
    &input,
    |b, lines| {
      b.iter(|| {
        for line in lines.clone() {
          black_box(
            irc_rust::Message::from_str(&line)
              .expect("failed to parse")
              .parse()
              .expect("failed to parse"),
          );
        }
      })
    },
  );
}

criterion_group!(
  parse_benches,
  parse_twitch,
  parse_twitch_irc,
  parse_irc_rust
);
criterion_main!(parse_benches);