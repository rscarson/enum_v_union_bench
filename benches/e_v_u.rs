use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone)]
enum EnumVersion<'a> {
    Path(&'a Path),
    Str(&'a str),
}
impl<'a> EnumVersion<'a> {
    fn as_path(&self) -> &Path {
        match self {
            EnumVersion::Path(p) => p,
            EnumVersion::Str(s) => Path::new(s),
        }
    }
}

#[derive(Clone)]
struct TaggedUnion<'a> {
    tag: u8,
    data: InnerUnion<'a>,
}
union InnerUnion<'a> {
    path: &'a Path,
    str: &'a str,
}
impl Copy for InnerUnion<'_> {}
impl Clone for InnerUnion<'_> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a> TaggedUnion<'a> {
    fn new() -> Self {
        TaggedUnion {
            tag: 0,
            data: InnerUnion {
                path: Path::new("foo/bar/baz"),
            },
        }
    }

    fn as_path(&self) -> &Path {
        if self.tag == 0 {
            unsafe { self.data.path }
        } else {
            Path::new(unsafe { self.data.str })
        }
    }
}

fn read_manye<'a>(p: &'a [EnumVersion<'a>]) -> Vec<&'a Path> {
    p.iter().map(|v| v.as_path()).collect()
}

fn read_manyu<'a>(p: &'a [TaggedUnion<'a>]) -> Vec<&'a Path> {
    p.iter().map(|v| v.as_path()).collect()
}

fn bench(c: &mut Criterion) {
    let enums = vec![EnumVersion::Path(Path::new("foo/bar/baz")); 10000];
    let unions = vec![TaggedUnion::new(); 10000];

    let mut group = c.benchmark_group("Path");
    group.bench_function("read_many_enum", |b| {
        b.iter(|| black_box(read_manye(&enums)))
    });
    group.bench_function("read_many_union", |b| {
        b.iter(|| black_box(read_manyu(&unions)))
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
