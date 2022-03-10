use sdl2::pixels::Color;

pub(crate) static BACKGROUND_COLOR: Color = Color::RGBA(0x44, 0x44, 0x44, 0xFF);
pub(crate) static BOARD_COLOR: Color = Color::RGB(0x33, 0x33, 0x33);

pub(crate) static COLORS: [Color; 5] = [
    Color::GREEN,
    Color::RED,
    Color::YELLOW,
    Color::CYAN,
    Color::MAGENTA,
];

pub(crate) fn rand_color() -> Color {
    let idx: usize = rand::random();
    return COLORS[idx % COLORS.len()];
}

// [[[[pixel;column];row];direction];shape]
pub(crate) static ITEMS: [[[[bool; 4]; 4]; 4]; 7] = [
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
    // J J J J
    [
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
        ],
        [
            [false, false, false, false],
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, false],
            [false, false, true, false],
        ],
    ],
    // L L L L
    [
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, true, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, true, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
    ],
];
