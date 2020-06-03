use criterion::{black_box, criterion_group, criterion_main, Criterion};

use concat_strs::concat_strs;

static DATE: &str = "2014-11-28";
static T: &str = "T";
static TIME: &str = "12:00:09Z";

fn concat_strs_benchmark(c: &mut Criterion) {
    c.bench_function("concat_strs_constants", |b| {
        b.iter(|| {
            let datetime = concat_strs![DATE, T, TIME];
            black_box(datetime);
        })
    });

    c.bench_function("concat_strs_literals", |b| {
        b.iter(|| {
            let datetime = concat_strs!["2014-11-28", "T", "12:00:09Z"];
            black_box(datetime);
        })
    });

    c.bench_function("concat_strs_literals_char", |b| {
        b.iter(|| {
            let datetime = concat_strs!["2014-11-28", 'T', "12:00:09Z"];
            black_box(datetime);
        })
    });
}

fn array_concat(c: &mut Criterion) {
    c.bench_function("array_concat", |b| {
        b.iter(|| {
            let datetime: &str = &[DATE, T, TIME].concat();
            black_box(datetime);
        })
    });
}

fn array_join(c: &mut Criterion) {
    c.bench_function("array_join", |b| {
        b.iter(|| {
            let datetime: &str = &[DATE, TIME].join(T);
            black_box(datetime);
        })
    });
}

fn array_join_long(c: &mut Criterion) {
    c.bench_function("array_join_long", |b| {
        b.iter(|| {
            let datetime: &str = &[DATE, T, TIME].join("");
            black_box(datetime);
        })
    });
}

fn collect_from_array_to_string(c: &mut Criterion) {
    let list = [DATE, T, TIME];
    c.bench_function("collect_from_array_to_string", |b| {
        b.iter(|| {
            let datetime: String = list.iter().map(|x| *x).collect();
            black_box(datetime);
        })
    });
}

fn collect_from_vec_to_string(c: &mut Criterion) {
    let list = vec![DATE, T, TIME];
    c.bench_function("collect_from_vec_to_string", |b| {
        b.iter(|| {
            let datetime: String = list.iter().map(|x| *x).collect();
            black_box(datetime);
        })
    });
}

fn format_macro(c: &mut Criterion) {
    c.bench_function("format_macro", |b| {
        b.iter(|| {
            let datetime: &str = &format!("{}{}{}", DATE, T, TIME);
            black_box(datetime);
        })
    });
}

#[cfg(unix)]
fn from_bytes(c: &mut Criterion) {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::slice;

    c.bench_function("from_bytes", |b| {
        b.iter(|| {
            let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

            let datetime = OsStr::from_bytes(bytes);
            black_box(datetime);
        })
    });
}

fn mut_string_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::new();
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        })
    });
}

fn mut_string_with_capacity_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_with_capacity_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(20);
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        })
    });
}

fn mut_string_with_too_little_capacity_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_with_too_little_capacity_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(2);
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        })
    });
}

fn mut_string_push_string(c: &mut Criterion) {
    c.bench_function("mut_string_push_string", |b| {
        b.iter(|| {
            let mut datetime = Vec::<String>::new();
            datetime.push(String::from(DATE));
            datetime.push(String::from(T));
            datetime.push(String::from(TIME));
            let datetime = datetime.join("");
            black_box(datetime);
        })
    });
}

fn mut_string_with_too_much_capacity_push_str(c: &mut Criterion) {
    c.bench_function("mut_string_with_too_much_capacity_push_str", |b| {
        b.iter(|| {
            let mut datetime = String::with_capacity(200);
            datetime.push_str(DATE);
            datetime.push_str(T);
            datetime.push_str(TIME);
            black_box(datetime);
        })
    });
}

fn string_from_all(c: &mut Criterion) {
    c.bench_function("string_from_all", |b| {
        b.iter(|| {
            let datetime: &str = &(String::from(DATE) + &String::from(T) + &String::from(TIME));
            black_box(datetime);
        })
    });
}

fn string_from_plus_op(c: &mut Criterion) {
    c.bench_function("string_from_plus_op", |b| {
        b.iter(|| {
            let datetime: &str = &(String::from(DATE) + T + TIME);
            black_box(datetime);
        })
    });
}

fn to_owned_plus_op(c: &mut Criterion) {
    c.bench_function("to_owned_plus_op", |b| {
        b.iter(|| {
            let datetime: &str = &(DATE.to_owned() + T + TIME);
            black_box(datetime);
        })
    });
}

fn to_string_plus_op(c: &mut Criterion) {
    c.bench_function("to_string_plus_op", |b| {
        b.iter(|| {
            let datetime: &str = &(DATE.to_string() + T + TIME);
            black_box(datetime);
        })
    });
}

criterion_group!(
    benches,
    concat_strs_benchmark,
    array_concat,
    array_join,
    array_join_long,
    collect_from_array_to_string,
    collect_from_vec_to_string,
    format_macro,
    from_bytes,
    mut_string_push_str,
    mut_string_push_string,
    mut_string_with_capacity_push_str,
    mut_string_with_too_little_capacity_push_str,
    mut_string_with_too_much_capacity_push_str,
    string_from_all,
    string_from_plus_op,
    to_owned_plus_op,
    to_string_plus_op
);
criterion_main!(benches);
