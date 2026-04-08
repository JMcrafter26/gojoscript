costumes "blank.svg";

list input = file ```input.txt```;
struct Rule { left, right }
list rules: Rule;
list pages;
list strsplitchar;

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

function strsplitchar(string, char) {
    delete strsplitchar;
    let part = "";
    let i = 1;
    repeat (length($string)) {
        if ($string[i] == $char) {
            add part to strsplitchar;
            part = "";
        } else {
            part &= $string[i];
        }
        i++;
    }
    if (length(part) > 0) {
        add part to strsplitchar;
    }
}

function parse_input() {
    delete rules;
    delete pages;
    let i = 1;
    repeat (length(input)) {
        strfindchar(input[i], char: "|");
        if (strfindchar > 0) {
            strsplitchar(input[i], "|");
            add Rule { left: strsplitchar[1], right: strsplitchar[2] } to rules;
        } else {
            strfindchar(input[i], char: ",");
            if (strfindchar > 0) {
                strsplitchar(input[i], char: ",");
                add length(strsplitchar) to pages;
                let j = 1;
                repeat (length(strsplitchar)) {
                    add strsplitchar[j] to pages;
                    j++;
                }
            }
        }
        i++;
    }
}

function page_find_idx(page_ptr, value) {
    page_find_idx = 0;
    let i = $page_ptr + 1;
    repeat (pages[$page_ptr]) {
        if (pages[i] == $value) {
            page_find_idx = i - $page_ptr;
            stop_this_script;
        }
        i++;
    }
}

function rule_in_page(rule: Rule, page_ptr) {
    rule_in_page = true;
    page_find_idx($page_ptr, $rule.left);
    let left_idx = page_find_idx;
    page_find_idx($page_ptr, $rule.right);
    let right_idx = page_find_idx;
    if (left_idx > 0 && right_idx > 0) {
        rule_in_page = left_idx < right_idx;
    }
}

function rules_in_page(page_ptr) {
    let i = 1;
    repeat (length(rules)) {
        rule_in_page(rules[i], $page_ptr);
        if (rule_in_page == false) {
            stop_this_script;
        }
        i++;
    }
}

function middle_number(page_ptr) {
    middle_number = pages[$page_ptr + 1 + pages[$page_ptr] div 2];
}

function main() {
    parse_input();
    let sum = 0;
    let i = 1;
    while (!(i > length(pages))) {
        rules_in_page(i);
        if (rule_in_page == true) {
            middle_number(i);
            sum += middle_number;
        }
        i += pages[i] + 1;
    }
    let result = 0;
    i = 1;
    while (!(i > length(pages))) {
        rules_in_page(i);
        if (rule_in_page == false) {
            while (!(rule_in_page == true)) {
                let j = 1;
                while (!(j > length(rules))) {
                    page_find_idx(i, rules[j].left);
                    let left_idx = page_find_idx;
                    page_find_idx(i, rules[j].right);
                    let right_idx = page_find_idx;
                    if (left_idx > 0 && right_idx > 0) {
                        if (left_idx > right_idx) {
                            let temp = pages[i + left_idx];
                            pages[i + left_idx] = pages[i + right_idx];
                            pages[i + right_idx] = temp;
                            j = length(rules);
                        }
                    }
                    j++;
                }
                rules_in_page(i);
            }
            middle_number(i);
            result += middle_number;
        }
        i += pages[i] + 1;
    }
    say "Result 1: " & sum & "\nResult 2: " & result;
}

onflag() {
    main();
}
