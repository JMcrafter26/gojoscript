costumes "blank.svg";

list input = file ```input.txt```;

function count_xmas() {
    count_xmas = 0;
    let i = 1;
    repeat (length(input)) {
        let j = 1;
        repeat (length(input[1])) {
            count_xmas += (
                    input[i][j    ] == "X"
                && input[i][j + 1] == "M"
                && input[i][j + 2] == "A"
                && input[i][j + 3] == "S"
            )
            + (
                    input[i][j    ] == "S"
                && input[i][j + 1] == "A"
                && input[i][j + 2] == "M"
                && input[i][j + 3] == "X"
            )
            + (
                    input[i    ][j] == "X"
                && input[i + 1][j] == "M"
                && input[i + 2][j] == "A"
                && input[i + 3][j] == "S"
            )
            + (
                    input[i    ][j] == "S"
                && input[i + 1][j] == "A"
                && input[i + 2][j] == "M"
                && input[i + 3][j] == "X"
            )
            + (
                    input[i    ][j    ] == "X"
                && input[i + 1][j + 1] == "M"
                && input[i + 2][j + 2] == "A"
                && input[i + 3][j + 3] == "S"
            )
            + (
                    input[i    ][j    ] == "S"
                && input[i + 1][j + 1] == "A"
                && input[i + 2][j + 2] == "M"
                && input[i + 3][j + 3] == "X"
            )
            + (
                    input[i    ][j + 3] == "X"
                && input[i + 1][j + 2] == "M"
                && input[i + 2][j + 1] == "A"
                && input[i + 3][j    ] == "S"
            )
            + (
                    input[i    ][j + 3] == "S"
                && input[i + 1][j + 2] == "A"
                && input[i + 2][j + 1] == "M"
                && input[i + 3][j    ] == "X"
            );
            j++;
        }
        i++;
    }
}

function count_x_mas() {
    count_x_mas = 0;
    let i = 1;
    repeat (length(input)) {
        let j = 1;
        repeat (length(input[1])) {
            let a = input[i    ][j    ];
            let b = input[i    ][j + 2];
            let c = input[i + 2][j    ];
            let d = input[i + 2][j + 2];
            count_x_mas += input[i + 1][j + 1] == "A" && ((
                    a == "M"
                && b == "S"
                && c == "M"
                && d == "S"
            ) || (
                    a == "S"
                && b == "M"
                && c == "S"
                && d == "M"
            ) || (
                    a == "M"
                && b == "M"
                && c == "S"
                && d == "S"
            ) || (
                    a == "S"
                && b == "S"
                && c == "M"
                && d == "M"
            ));
            j++;
        }
        i++;
    }
}

onflag() {
    count_xmas();
    count_x_mas();
    say "Count XMAS: " & count_xmas & "\nCount X-MAS: " & count_x_mas;
}
