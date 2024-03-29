use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gobit::{Color, Goban};

const LIAN_XIAO_KANG_DONGYUN: [(char, (u8, u8)); 166] = [
    ('B', (16, 3)), ('W', (3, 3)), ('B', (15, 16)), ('W', (3, 15)), ('B', (13, 3)), ('W', (15, 14)), ('B', (16, 14)),
    ('W', (16, 13)), ('B', (16, 15)), ('W', (15, 13)), ('B', (13, 16)), ('W', (15, 9)), ('B', (2, 16)), ('W', (2, 15)),
    ('B', (3, 16)), ('W', (4, 15)), ('B', (4, 16)), ('W', (5, 15)), ('B', (5, 16)), ('W', (6, 16)), ('B', (5, 2)),
    ('W', (2, 5)), ('B', (6, 15)), ('W', (7, 16)), ('B', (1, 16)), ('W', (6, 14)), ('B', (16, 7)), ('W', (14, 7)),
    ('B', (12, 9)), ('W', (9, 11)), ('B', (14, 8)), ('W', (15, 8)), ('B', (15, 7)), ('W', (13, 7)), ('B', (14, 9)),
    ('W', (15, 10)), ('B', (14, 10)), ('W', (16, 5)), ('B', (17, 5)), ('W', (11, 7)), ('B', (10, 9)), ('W', (11, 2)),
    ('B', (12, 4)), ('W', (14, 2)), ('B', (13, 2)), ('W', (15, 4)), ('B', (16, 4)), ('W', (15, 5)), ('B', (17, 6)),
    ('W', (15, 3)), ('B', (16, 2)), ('W', (13, 1)), ('B', (12, 1)), ('W', (15, 1)), ('B', (13, 0)), ('W', (14, 1)),
    ('B', (12, 2)), ('W', (13, 5)), ('B', (17, 9)), ('W', (10, 4)), ('B', (9, 2)), ('W', (12, 14)), ('B', (15, 11)),
    ('W', (16, 11)), ('B', (14, 11)), ('W', (16, 10)), ('B', (10, 12)), ('W', (14, 15)), ('B', (14, 16)),
    ('W', (10, 14)), ('B', (9, 12)), ('W', (10, 2)), ('B', (9, 3)), ('W', (11, 4)), ('B', (10, 1)), ('W', (9, 8)),
    ('B', (12, 5)), ('W', (12, 6)), ('B', (10, 6)), ('W', (9, 9)), ('B', (10, 11)), ('W', (8, 11)), ('B', (13, 13)),
    ('W', (12, 13)), ('B', (12, 12)), ('W', (13, 12)), ('B', (14, 13)), ('W', (11, 16)), ('B', (11, 5)), ('W', (10, 8)),
    ('B', (9, 14)), ('W', (9, 15)), ('B', (10, 15)), ('W', (11, 15)), ('B', (8, 15)), ('W', (10, 16)), ('B', (8, 12)),
    ('W', (7, 12)), ('B', (7, 13)), ('W', (6, 13)), ('B', (8, 6)), ('W', (11, 1)), ('B', (9, 4)), ('W', (1, 15)),
    ('B', (10, 10)), ('W', (3, 8)), ('B', (5, 9)), ('W', (4, 2)), ('B', (3, 9)), ('W', (4, 9)), ('B', (4, 10)),
    ('W', (4, 8)), ('B', (3, 10)), ('W', (5, 10)), ('B', (2, 8)), ('W', (2, 7)), ('B', (1, 7)), ('W', (1, 8)),
    ('B', (2, 9)), ('W', (1, 9)), ('B', (3, 7)), ('W', (2, 6)), ('B', (1, 10)), ('W', (1, 6)), ('B', (5, 11)),
    ('W', (6, 10)), ('B', (6, 11)), ('W', (5, 8)), ('B', (2, 11)), ('W', (7, 11)), ('B', (4, 12)), ('W', (9, 1)),
    ('B', (8, 1)), ('W', (10, 0)), ('B', (12, 0)), ('W', (7, 1)), ('B', (7, 2)), ('W', (8, 0)), ('B', (8, 2)),
    ('W', (5, 1)), ('B', (6, 1)), ('W', (6, 0)), ('B', (7, 0)), ('W', (1, 12)), ('B', (4, 1)), ('W', (3, 12)),
    ('B', (8, 16)), ('W', (7, 15)), ('B', (8, 14)), ('W', (17, 10)), ('B', (2, 12)), ('W', (3, 13)), ('B', (17, 12)),
    ('W', (17, 13)), ('B', (3, 2)), ('W', (4, 3)), ('B', (2, 2)), ('W', (15, 15)), ('B', (2, 3)), ('W', (5, 3)),
    ('B', (1, 13)), ('W', (3, 11)), ('B', (4, 11)), ('W', (6, 9)), ('B', (0, 14)), ('W', (0, 15))
];

const LI_XUANHAO_SHIN_JINSEO: [(char, (u8, u8)); 170] = [
    ('B', (15, 3)), ('W', (3, 15)), ('B', (15, 16)), ('W', (3, 2)), ('B', (2, 16)), ('W', (2, 15)), ('B', (3, 16)),
    ('W', (5, 16)), ('B', (5, 17)), ('W', (6, 17)), ('B', (4, 16)), ('W', (5, 15)), ('B', (4, 15)), ('W', (4, 14)),
    ('B', (3, 14)), ('W', (1, 14)), ('B', (3, 13)), ('W', (4, 13)), ('B', (4, 12)), ('W', (5, 12)), ('B', (2, 12)),
    ('W', (4, 11)), ('B', (3, 12)), ('W', (1, 16)), ('B', (4, 17)), ('W', (2, 10)), ('B', (5, 11)), ('W', (5, 13)),
    ('B', (6, 16)), ('W', (4, 10)), ('B', (3, 3)), ('W', (4, 2)), ('B', (2, 3)), ('W', (1, 11)), ('B', (2, 2)),
    ('W', (13, 2)), ('B', (6, 15)), ('W', (6, 11)), ('B', (7, 2)), ('W', (4, 3)), ('B', (10, 3)), ('W', (4, 5)),
    ('B', (2, 6)), ('W', (9, 5)), ('B', (7, 6)), ('W', (8, 4)), ('B', (5, 10)), ('W', (7, 10)), ('B', (5, 9)),
    ('W', (4, 9)), ('B', (5, 8)), ('W', (4, 8)), ('B', (7, 13)), ('W', (7, 12)), ('B', (4, 7)), ('W', (2, 5)),
    ('B', (7, 4)), ('W', (7, 3)), ('B', (6, 3)), ('W', (8, 3)), ('B', (1, 5)), ('W', (6, 2)), ('B', (8, 12)),
    ('W', (5, 7)), ('B', (6, 1)), ('W', (5, 2)), ('B', (8, 2)), ('W', (9, 2)), ('B', (9, 6)), ('W', (9, 1)),
    ('B', (6, 7)), ('W', (5, 6)), ('B', (7, 8)), ('W', (9, 10)), ('B', (8, 5)), ('W', (10, 5)), ('B', (8, 11)),
    ('W', (7, 9)), ('B', (10, 6)), ('W', (11, 5)), ('B', (11, 6)), ('W', (12, 5)), ('B', (12, 6)), ('W', (13, 5)),
    ('B', (8, 10)), ('W', (8, 9)), ('B', (9, 9)), ('W', (8, 8)), ('B', (8, 7)), ('W', (9, 8)), ('B', (10, 9)),
    ('W', (10, 8)), ('B', (11, 8)), ('W', (11, 9)), ('B', (10, 10)), ('W', (12, 8)), ('B', (11, 7)), ('W', (1, 12)),
    ('B', (5, 14)), ('W', (6, 12)), ('B', (11, 10)), ('W', (12, 9)), ('B', (12, 10)), ('W', (15, 14)), ('B', (13, 15)),
    ('W', (13, 14)), ('B', (16, 15)), ('W', (14, 11)), ('B', (14, 9)), ('W', (12, 15)), ('B', (14, 15)),
    ('W', (12, 13)), ('B', (16, 12)), ('W', (13, 10)), ('B', (13, 9)), ('W', (12, 11)), ('B', (13, 8)), ('W', (12, 16)),
    ('B', (14, 2)), ('W', (13, 1)), ('B', (13, 3)), ('W', (12, 3)), ('B', (11, 14)), ('W', (12, 14)), ('B', (10, 16)),
    ('W', (10, 15)), ('B', (9, 16)), ('W', (16, 14)), ('B', (14, 14)), ('W', (14, 13)), ('B', (15, 13)),
    ('W', (15, 12)), ('B', (16, 13)), ('W', (16, 11)), ('B', (17, 14)), ('W', (17, 10)), ('B', (13, 12)),
    ('W', (15, 11)), ('B', (6, 13)), ('W', (6, 10)), ('B', (6, 8)), ('W', (2, 14)), ('B', (11, 13)), ('W', (10, 12)),
    ('B', (10, 13)), ('W', (9, 13)), ('B', (9, 14)), ('W', (17, 7)), ('B', (17, 6)), ('W', (16, 7)), ('B', (14, 1)),
    ('W', (17, 3)), ('B', (16, 4)), ('W', (17, 4)), ('B', (17, 5)), ('W', (16, 1)), ('B', (15, 6)), ('W', (15, 1)),
    ('B', (16, 9)), ('W', (17, 9)), ('B', (16, 8)), ('W', (17, 8)), ('B', (13, 0)), ('W', (14, 6)), ('B', (15, 7)),
    ('W', (16, 6)), ('B', (16, 5)), ('W', (15, 5)), ('B', (12, 4)), ('W', (13, 4))
];

const YANG_DINGXIN_SHIBANO_TORAMARU: [(char, (u8, u8)); 172] = [
    ('B', (15, 3)), ('W', (3, 15)), ('B', (15, 15)), ('W', (3, 2)), ('B', (2, 4)), ('W', (4, 3)), ('B', (2, 16)),
    ('W', (2, 15)), ('B', (3, 16)), ('W', (4, 15)), ('B', (5, 16)), ('W', (6, 14)), ('B', (6, 17)), ('W', (2, 7)),
    ('B', (13, 2)), ('W', (4, 7)), ('B', (16, 8)), ('W', (12, 3)), ('B', (13, 3)), ('W', (12, 5)), ('B', (13, 16)),
    ('W', (10, 2)), ('B', (1, 2)), ('W', (1, 6)), ('B', (7, 13)), ('W', (7, 14)), ('B', (9, 13)), ('W', (6, 11)),
    ('B', (6, 2)), ('W', (5, 2)), ('B', (3, 6)), ('W', (3, 7)), ('B', (5, 4)), ('W', (4, 4)), ('B', (5, 1)),
    ('W', (7, 2)), ('B', (4, 1)), ('W', (6, 3)), ('B', (3, 1)), ('W', (6, 1)), ('B', (4, 2)), ('W', (17, 14)),
    ('B', (17, 15)), ('W', (16, 6)), ('B', (5, 3)), ('W', (17, 3)), ('B', (6, 2)), ('W', (16, 2)), ('B', (14, 6)),
    ('W', (5, 2)), ('B', (6, 4)), ('W', (16, 14)), ('B', (16, 15)), ('W', (16, 10)), ('B', (17, 5)), ('W', (16, 4)),
    ('B', (11, 3)), ('W', (14, 5)), ('B', (15, 5)), ('W', (16, 5)), ('B', (11, 2)), ('W', (15, 4)), ('B', (7, 4)),
    ('W', (9, 4)), ('B', (13, 5)), ('W', (15, 6)), ('B', (13, 4)), ('W', (14, 7)), ('B', (13, 6)), ('W', (10, 3)),
    ('B', (11, 1)), ('W', (9, 6)), ('B', (6, 7)), ('W', (6, 9)), ('B', (14, 8)), ('W', (13, 7)), ('B', (12, 6)),
    ('W', (9, 8)), ('B', (15, 7)), ('W', (4, 6)), ('B', (6, 2)), ('W', (1, 16)), ('B', (1, 17)), ('W', (5, 2)),
    ('B', (8, 3)), ('W', (10, 1)), ('B', (6, 2)), ('W', (13, 8)), ('B', (14, 9)), ('W', (5, 2)), ('B', (10, 5)),
    ('W', (9, 5)), ('B', (6, 2)), ('W', (16, 7)), ('B', (15, 8)), ('W', (5, 2)), ('B', (7, 3)), ('W', (11, 8)),
    ('B', (14, 11)), ('W', (13, 10)), ('B', (14, 10)), ('W', (15, 12)), ('B', (17, 9)), ('W', (11, 7)), ('B', (12, 12)),
    ('W', (6, 2)), ('B', (3, 3)), ('W', (4, 5)), ('B', (8, 14)), ('W', (17, 10)), ('B', (15, 11)), ('W', (16, 12)),
    ('B', (15, 13)), ('W', (14, 12)), ('B', (13, 11)), ('W', (13, 12)), ('B', (12, 11)), ('W', (15, 14)),
    ('B', (14, 13)), ('W', (14, 15)), ('B', (14, 16)), ('W', (13, 14)), ('B', (17, 12)), ('W', (17, 11)),
    ('B', (13, 13)), ('W', (18, 12)), ('B', (16, 13)), ('W', (17, 13)), ('B', (16, 11)), ('W', (17, 12)),
    ('B', (12, 13)), ('W', (18, 10)), ('B', (6, 13)), ('W', (5, 13)), ('B', (5, 12)), ('W', (6, 12)), ('B', (5, 14)),
    ('W', (4, 13)), ('B', (5, 15)), ('W', (1, 15)), ('B', (17, 7)), ('W', (18, 14)), ('B', (8, 1)), ('W', (8, 2)),
    ('B', (9, 3)), ('W', (10, 4)), ('B', (15, 1)), ('W', (15, 2)), ('B', (14, 1)), ('W', (16, 1)), ('B', (9, 2)),
    ('W', (9, 1)), ('B', (6, 0)), ('W', (12, 4)), ('B', (17, 6)), ('W', (14, 4)), ('B', (2, 12)), ('W', (3, 11)),
    ('B', (5, 8)), ('W', (8, 12)), ('B', (8, 13)), ('W', (6, 5)), ('B', (2, 11)), ('W', (3, 10)), ('B', (2, 10)),
    ('W', (3, 13)), ('B', (3, 9)), ('W', (4, 9)), ('B', (4, 10)), ('W', (4, 11)), ('B', (4, 8)), ('W', (7, 5))
];

fn playout(moves: &[(char, (u8, u8))]) -> Goban {
    let mut goban = Goban::new(19, 19);

    for (color, (x, y)) in moves {
        let at = (*x, *y).into();
        let color = match color {
            'B' => Color::Black,
            'W' => Color::White,
            _ => unreachable!(),
        };

        debug_assert!(goban.is_legal(at, color));
        goban.play(at, color);
    }

    goban
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lian_xiao_kang_dongyun", |b| b.iter(|| playout(black_box(&LIAN_XIAO_KANG_DONGYUN))));
    c.bench_function("li_xuanhao_shin_jinseo", |b| b.iter(|| playout(black_box(&LI_XUANHAO_SHIN_JINSEO))));
    c.bench_function("yang_dingxin_shibano_toramaru", |b| b.iter(|| playout(black_box(&YANG_DINGXIN_SHIBANO_TORAMARU))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
