use sdl2::pixels::Color;

pub(crate) static colors: [Color; 5] = [
    Color::GREEN,
    Color::RED,
    Color::BLUE,
    Color::YELLOW,
    Color::CYAN,
];

pub(crate) fn rand_color() -> Color {
    let idx: usize = rand::random();
    return colors[idx % colors.len()];
}

// [[[[pixel;column];row];direction];shape]
pub(crate) static items: [[[[bool; 4]; 4]; 4]; 5] = [
    // — | — |
    [
        [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
    ],
    // ▩ ▩ ▩ ▩
    [
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
    ],
    // ┷ ┠ ┯ ┨
    [
        // ┷
        [
            [false, false, false, false],
            [false, true, false, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
        // ┠
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false, true, false, false],
        ],
        // ┯
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
        ],
        // ┨
        [
            [false, false, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false, true, false, false],
        ],
    ],
    // S S S S
    [
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
        ],
    ],
    // Z Z Z Z
    [
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [true, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [true, false, false, false],
        ],
    ],
];
