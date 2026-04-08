costumes "blank.svg";

list input = file ```input.txt```;
list split;
list reports;

function split(string, sep) {
    delete split;
    let part = "";
    let i = 1;
    repeat (length($string)) {
        if ($string[i] == $sep) {
            add part to split;
            part = "";
        } else {
            part &= $string[i];
        }
        i++;
    }
    if (length(part) > 0) {
        add part to split;
    }
}

function parse_input() {
    delete reports;
    let i = 1;
    repeat (length(input)) {
        split(input[i], sep: " ");
        add length(split) to reports;
        let j = 1;
        repeat (length(split)) {
            add split[j] to reports;
            j++;
        }
        i++;
    }
}

function is_report_safe(idx, skip_idx) {
    is_report_safe = false;
    let i = $idx + 1;
    let dir = 0;
    if ($skip_idx == 1) {
        i++;
    }
    while (!(i - $idx + 1 > reports[$idx] - ($skip_idx == reports[$idx]))) {
        if (i + 1 == $idx + $skip_idx) {
            let difference = reports[i + 2] - reports[i];
            i++;
        } else {
            let difference = reports[i + 1] - reports[i];
        }
        let distance = abs(difference);
        let newdir = difference / distance;
        if (distance < 1 || distance > 3 || (dir != 0 && dir != newdir)) {
            stop_this_script;
        }
        dir = newdir;
        i++;
    }
    is_report_safe = true;
}

function is_report_safe_with_problem_dampener(idx) {
    let i = 1;
    repeat (reports[$idx]) {
        is_report_safe($idx, i);
        if (is_report_safe == true) {
            stop_this_script;
        }
        i++;
    }
}

function count_safe_reports() {
    count_safe_reports = 0;
    let i = 1;
    while (!(i > length(reports))) {
        is_report_safe(i, skip_idx: 0);
        if (is_report_safe == true) {
            count_safe_reports++;
        }
        i += reports[i] + 1;
    }
}

function count_safe_reports_with_problem_dampener() {
    count_safe_reports_with_problem_dampener = 0;
    let i = 1;
    while (!(i > length(reports))) {
        is_report_safe(i, skip_idx: 0);
        if (is_report_safe == false) {
            is_report_safe_with_problem_dampener(i);
        }
        count_safe_reports_with_problem_dampener += is_report_safe;
        i += reports[i] + 1;
    }
}

onflag() {
    parse_input();
    count_safe_reports();
    count_safe_reports_with_problem_dampener();
    say
        "Safe reports: "
        & count_safe_reports
        & "\nSafe reports with problem dampener: "
        & count_safe_reports_with_problem_dampener;
}
