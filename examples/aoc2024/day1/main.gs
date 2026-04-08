costumes "blank.svg";

list input = file ```input.txt```;
list list1;
list list2;

function split_once(string, sep) {
    let i = 1;
    split_once_left = "";
    while (!($string[i] == $sep || i > length($string))) {
        split_once_left &= $string[i];
        i++;
    }
    i++;
    while (!($string[i] != $sep || i > length($string))) {
        i++;
    }
    split_once_right = "";
    while (!(i > length($string))) {
        split_once_right &= $string[i];
        i++;
    }
}

function parse_input() {
    delete list1;
    delete list2;
    let i = 1;
    repeat (length(input)) {
        split_once(input[i], sep: " ");
        add split_once_left to list1;
        add split_once_right to list2;
        i++;
    }
}

function sort_list1() {
    let i = 2;
    while (!(i > length(list1))) {
        let x = list1[i];
        let j = i;
        while (!(j <= 1 || list1[j - 1] <= x)) {
            list1[j] = list1[j - 1];
            j--;
        }
        list1[j] = x;
        i++;
    }
}

function sort_list2() {
    let i = 2;
    while (!(i > length(list2))) {
        let x = list2[i];
        let j = i;
        while (!(j <= 1 || list2[j - 1] <= x)) {
            list2[j] = list2[j - 1];
            j--;
        }
        list2[j] = x;
        i++;
    }
}

function count_list2(value) {
    count_list2 = 0;
    let i = 1;
    repeat (length(list2)) {
        if (list2[i] == $value) {
            count_list2 += 1;
        }
        i++;
    }
}

function get_total_distance() {
    total_distance = 0;
    let i = 1;
    repeat (length(list1)) {
        total_distance += abs(list1[i] - list2[i]);
        i++;
    }
}

function get_similarity_score() {
    similarity_score = 0;
    let i = 1;
    repeat (length(list1)) {
        count_list2(list1[i]);
        similarity_score += list1[i] * count_list2;
        i++;
    }
}

onflag() {
    parse_input();
    sort_list1();
    sort_list2();
    get_total_distance();
    get_similarity_score();
    say "Total Distance: " & total_distance & "\nSimilarity Score: " & similarity_score;
}
