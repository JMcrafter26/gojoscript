costumes "blank.svg";

list input = file ```input.txt```;

function strsetchar(string, index, char) {
    strsetchar = "";
    let i = 1;
    repeat (length($string)) {
        if (i == $index) {
            strsetchar &= $char;
        } else {
            strsetchar &= $string[i];
        }
        i++;
    }
}

function strfindchar(string, char) {
    strfindchar = 0;
    let i = 1;
    repeat (length($string)) {
        if ($string[i] == $char) {
            strfindchar = i;
        }
        i++;
    }
}

function step() {
    let nx = x + cos(dir);
    let ny = y + sin(dir);
    if (input[ny][nx] == "#") {
        dir += 90;
    } else {
        x = nx;
        y = ny;
        strsetchar(input[ny], nx, char: "X");
        input[ny] = strsetchar;
    }
}

function run() {
    while (!(x < 0 || y < 0 || x > length(input[1]) || y > length(input))) {
        step();
    }
}

function count_x() {
    count_x = 1;
    let i = 1;
    repeat (length(input)) {
        let j = 1;
        repeat (length(input[1])) {
            if (input[i][j] == "X") {
                count_x++;
            }
            j++;
        }
        i++;
    }
}

function find_guard() {
    let i = 1;
    repeat (length(input)) {
        let j = 1;
        repeat (length(input[1])) {
            if (input[i][j] == "^") {
                x = j;
                y = i;
            }
            j++;
        }
        i++;
    }
}

onflag() {
    find_guard();
    dir = 0;
    run();
    count_x();
    say "Count X: " & count_x;
}
