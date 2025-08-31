use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xml_nom_parse::types::*;

pub fn owned_bench(c: &mut Criterion) {
    let data_as_utf8 = String::from_utf8(std::fs::read("map.tmx").unwrap()).unwrap();

    c.bench_function("TMX ", |b| b.iter(|| Xml::from_input_str(data_as_utf8.as_str()).unwrap()));
}

pub fn ref_bench(c: &mut Criterion) {
    let data_as_utf8 = String::from_utf8(std::fs::read("map.tmx").unwrap()).unwrap();

    c.bench_function("TMX Ref ", |b| b.iter(|| XmlRef::from_input_str(data_as_utf8.as_str()).unwrap()));
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .sample_size(500)
        .measurement_time(std::time::Duration::from_secs(20));
    targets = owned_bench, ref_bench
}
criterion_main!(benches);
