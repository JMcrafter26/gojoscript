costumes "blank.svg";

list input = file ```input.txt```;

function join_input() {
    join_input = "";
    let i = 1;
    repeat (length(input)) {
        join_input &= input[i];
        i++;
    }
}

function run(text, enable_do_donts) {
    run = 0;
    let do = true;
    let i = 1;
    while (!(i > length($text))) {
        if ($text[i] == "d") {
            i++;
            if ($text[i] == "o") {
                i++;
                if ($text[i] == "(") {
                    i++;
                    if ($text[i] == ")") {
                        i++;
                        do = true;
                    }
                } else if ($text[i] == "n") {
                    i++;
                    if ($text[i] == "'") {
                        i++;
                        if ($text[i] == "t") {
                            i++;
                            if ($text[i] == "(") {
                                i++;
                                if ($text[i] == ")") {
                                    i++;
                                    do = false;
                                }
                            }
                        }
                    }
                }
            }
        } else if ($text[i] == "m") {
            i++;
            if ($text[i] == "u") {
                i++;
                if ($text[i] == "l") {
                    i++;
                    if ($text[i] == "(") {
                        i++;
                        let x = "";
                        while (!($text[i] * 1 != $text[i])) {
                            x &= $text[i];
                            i++;
                        }
                        if (x != "" && $text[i] == ",") {
                            i++;
                            let y = "";
                            while (!($text[i] * 1 != $text[i])) {
                                y &= $text[i];
                                i++;
                            }
                            if (y != "" && $text[i] == ")") {
                                i++;
                                if ($enable_do_donts == false || do == true) {
                                    run += x * y;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            i++;
        }
    }
}

onflag() {
    join_input();
    run(join_input, enable_do_donts: false);
    without_do_donts = run;
    run(join_input, enable_do_donts: true);
    say
        "Without do() and don't() enabled: "
        & without_do_donts
        & "\nWith do() and don't() enabled: "
        & run;
}
