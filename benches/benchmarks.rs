use criterion::*;
use ecs_bench_suite::*;

fn bench_simple_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_insert");
    group.bench_function("legion", |b| {
        let mut bench = legion::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::simple_insert::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("hecs", |b| {
    //     let mut bench = hecs::simple_insert::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::simple_insert::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::simple_insert::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_simple_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_iter");
    group.bench_function("legion", |b| {
        let mut bench = legion::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion (packed)", |b| {
    //     let mut bench = legion_packed::simple_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::simple_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy_query", |b| {
        let mut bench = bevy::simple_iter_query::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy_input", |b| {
        let mut bench = bevy::simple_iter_input::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("hecs", |b| {
    //     let mut bench = hecs::simple_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::simple_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard (packed)", |b| {
    //     let mut bench = shipyard_packed::simple_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::simple_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_frag_iter_bc(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmented_iter");
    group.bench_function("legion", |b| {
        let mut bench = legion::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::frag_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });

    group.bench_function("bevy_query", |b| {
        let mut bench = bevy::frag_iter_query::Benchmark::new();
        b.iter(move || bench.run());
    });

    // group.bench_function("bevy_foreach", |b| {
    //     let mut bench = bevy::frag_iter_foreach::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("hecs", |b| {
    //     let mut bench = hecs::frag_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::frag_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::frag_iter::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_schedule(c: &mut Criterion) {
    let mut group = c.benchmark_group("schedule");
    group.bench_function("legion", |b| {
        let mut bench = legion::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion (packed)", |b| {
    //     let mut bench = legion_packed::schedule::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::schedule::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::schedule::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard (packed)", |b| {
    //     let mut bench = shipyard_packed::schedule::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::schedule::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_heavy_compute(c: &mut Criterion) {
    let mut group = c.benchmark_group("heavy_compute");
    group.bench_function("legion", |b| {
        let mut bench = legion::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion (packed)", |b| {
    //     let mut bench = legion_packed::heavy_compute::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::heavy_compute::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("hecs", |b| {
    //     let mut bench = hecs::heavy_compute::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::heavy_compute::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard (packed)", |b| {
    //     let mut bench = shipyard_packed::heavy_compute::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::heavy_compute::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_add_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove_component");
    group.bench_function("legion", |b| {
        let mut bench = legion::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::add_remove::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("hecs", |b| {
    //     let mut bench = hecs::add_remove::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::add_remove::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::add_remove::Benchmark::new();
    //     b.iter(move || bench.run());
    // });

    // todo Bevy appears to crash in this benchmark
    group.bench_function("bevy", |b| {
        let mut bench = bevy::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_get_component(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_component");
    group.bench_function("legion", |b| {
        let mut bench = legion::get_component::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion_system", |b| {
        let mut bench = legion::get_component_system::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("legion_0.2.4", |b| {
    //     let mut bench = legion_2_4::get_component::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::get_component::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy_query", |b| {
        let mut bench = bevy::get_component_query::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("hecs", |b| {
    //     let mut bench = hecs::get_component::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("shipyard", |b| {
    //     let mut bench = shipyard::simple_insert::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
    // group.bench_function("specs", |b| {
    //     let mut bench = specs::simple_insert::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_serialize_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_text");
    group.bench_function("legion", |b| {
        let mut bench = legion::serialize_text::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("bevy", |b| {
    //     let mut bench = bevy::serialize_text::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

fn bench_serialize_binary(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize_binary");
    group.bench_function("legion", |b| {
        let mut bench = legion::serialize_binary::Benchmark::new();
        b.iter(move || bench.run());
    });
    // group.bench_function("bevy", |b| {
    //     let mut bench = bevy::serialize_text::Benchmark::new();
    //     b.iter(move || bench.run());
    // });
}

criterion_group!(
    benchmarks,
    bench_simple_insert,
    bench_simple_iter,
    bench_frag_iter_bc,
    bench_schedule,
    bench_heavy_compute,
    bench_add_remove,
    bench_get_component,
    // bench_serialize_text,
    // bench_serialize_binary,
);
criterion_main!(benchmarks);
