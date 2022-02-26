#[macro_use]
extern crate criterion;
use criterion::{AxisScale, Criterion, PlotConfiguration};

mod rust_tle;

fn compare(c: &mut Criterion) {
    let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791";

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Parse");
    group.plot_config(plot_config);

    group.bench_with_input("tle_parser::parse", tle_string, |b, tle_string| {
        b.iter(|| tle_parser::parse(tle_string).unwrap())
    });

    group.bench_with_input("tle::parse_tle", tle_string, |b, tle_string| {
        b.iter(|| rust_tle::parse(tle_string))
    });

    group.bench_with_input("tletools::parse", tle_string, |b, tle_string| {
        b.iter(|| tletools::parse(tle_string).unwrap())
    });

    group.bench_with_input("tletools::nom", tle_string, |b, tle_string| {
        b.iter(|| tletools::nom::parse_single_tle(tle_string).unwrap())
    });

    group.bench_with_input("sgp4::parse_3les", tle_string, |b, tle_string| {
        b.iter(|| sgp4::parse_3les(tle_string).unwrap())
    });

    group.bench_with_input("sgp4::Elements::from_tle", tle_string, |b, tle_string| {
        b.iter(|| {
            let mut lines = tle_string.lines();
            let name = lines.next().unwrap();
            let line1 = lines.next().unwrap();
            let line2 = lines.next().unwrap();
            sgp4::Elements::from_tle(Some(name.to_string()), line1.as_bytes(), line2.as_bytes())
                .unwrap()
        })
    });

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
