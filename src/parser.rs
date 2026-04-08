use grammar::SpriteParser;
use lalrpop_util::lalrpop_mod;

use crate::{
    ast::Sprite,
    diagnostic::Diagnostic,
    lexer::{
        adaptor,
        token::Token,
    },
    pre_processor::PreProcessor,
    translation_unit::TranslationUnit,
};

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    grammar,
    "/parser/grammar.rs"
);

type SpannedToken = (usize, Token, usize);

/// Tokenize the source code from a translation unit
fn tokenize(translation_unit: &TranslationUnit) -> (Vec<SpannedToken>, Vec<Diagnostic>) {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();

    adaptor::Lexer::new(std::str::from_utf8(&translation_unit.text).unwrap()).for_each(|result| {
        match result {
            Ok(token) => tokens.push(token),
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    });

    (tokens, diagnostics)
}

/// Apply preprocessing to the tokens
fn preprocess(mut tokens: Vec<SpannedToken>) -> (Vec<SpannedToken>, Option<Diagnostic>) {
    match PreProcessor::apply(&mut tokens) {
        Ok(()) => (tokens, None),
        Err(diagnostic) => (tokens, Some(diagnostic)),
    }
}

/// Parse the tokens into a sprite AST
fn parse_sprite(tokens: Vec<SpannedToken>) -> (Sprite, Vec<Diagnostic>) {
    let parser = SpriteParser::new();
    let mut sprite = Sprite::default();
    let mut diagnostics = Vec::new();

    if let Err(parse_error) = parser.parse(&mut sprite, &mut diagnostics, tokens) {
        diagnostics.push(parse_error.into());
    }

    (sprite, diagnostics)
}

/// Parse a translation unit into a sprite AST
///
/// This function performs the complete parsing pipeline:
/// 1. Tokenizes the source code
/// 2. Applies preprocessing transformations
/// 3. Parses the tokens into an AST
///
/// Returns the parsed sprite and any diagnostics encountered during parsing.
pub fn parse(translation_unit: &TranslationUnit) -> (Sprite, Vec<Diagnostic>) {
    let (tokens, tokenize_diagnostics) = tokenize(translation_unit);
    let (tokens, preprocess_diagnostic) = preprocess(tokens);
    let (sprite, parse_diagnostics) = parse_sprite(tokens);

    let all_diagnostics = tokenize_diagnostics
        .into_iter()
        .chain(preprocess_diagnostic)
        .chain(parse_diagnostics)
        .collect();

    (sprite, all_diagnostics)
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io::Cursor,
        rc::Rc,
    };

    use semver::Version;

    use crate::{
        codegen::sb3::Sb3,
        frontend::build::build_impl,
        standard_library::StandardLibrary,
        vfs::MemFS,
    };

    /// Build a goboscript project from in-memory source files and return whether
    /// compilation succeeded (no error-level diagnostics) and the .sb3 bytes.
    fn compile(files: &[(&str, &str)]) -> (bool, Vec<u8>) {
        let mut fs = MemFS::new();
        for (path, content) in files {
            fs.insert_file(*path, content.as_bytes());
        }
        let fs = Rc::new(RefCell::new(fs));
        let mut output = Vec::new();
        let sb3 = Sb3::new(Cursor::new(&mut output), fs.clone(), "project".into(), false);
        let stdlib = StandardLibrary {
            path: "stdlib".into(),
            version: Version::new(0, 0, 0),
        };
        let artifact = build_impl(fs, "project".into(), sb3, Some(stdlib)).unwrap();
        let success = !artifact.failure();
        (success, output)
    }

    /// Assert that a goboscript project compiles without errors.
    fn assert_compiles(files: &[(&str, &str)]) {
        let mut fs = MemFS::new();
        for (path, content) in files {
            fs.insert_file(*path, content.as_bytes());
        }
        let fs = Rc::new(RefCell::new(fs));
        let mut output = Vec::new();
        let sb3 = Sb3::new(Cursor::new(&mut output), fs.clone(), "project".into(), false);
        let stdlib = StandardLibrary {
            path: "stdlib".into(),
            version: Version::new(0, 0, 0),
        };
        let artifact = build_impl(fs, "project".into(), sb3, Some(stdlib)).unwrap();
        if artifact.failure() {
            let mut errors = vec![];
            for diag in &artifact.stage_diagnostics.diagnostics {
                errors.push(format!("[stage] {:?}", diag.kind));
            }
            for (name, sd) in &artifact.sprites_diagnostics {
                for diag in &sd.diagnostics {
                    errors.push(format!("[{}] {:?}", name, diag.kind));
                }
            }
            panic!("Expected project to compile without errors, but got:\n{}", errors.join("\n"));
        }
    }

    /// Assert that a goboscript project produces at least one error diagnostic.
    fn assert_fails(files: &[(&str, &str)]) {
        let (success, _) = compile(files);
        assert!(!success, "Expected project to fail compilation");
    }

    // ─── Minimal project helpers ───────────────────────────────────────────────

    const BLANK_SVG: &str = r#"<svg version="1.1" width="0" height="0" viewBox="0 0 0 0" xmlns="http://www.w3.org/2000/svg"></svg>"#;

    fn stage() -> (&'static str, &'static str) {
        ("project/stage.gs", "costumes \"blank.svg\";\n")
    }

    fn sprite<'a>(src: &'a str) -> [(&'a str, &'a str); 3] {
        [
            stage(),
            ("project/main.gs", src),
            ("project/blank.svg", BLANK_SVG),
        ]
    }

    // ─── Comment syntax ────────────────────────────────────────────────────────

    #[test]
    fn test_line_comment() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
// This is a line comment
onflag() {
    say "hello"; // inline comment
}
"#,
        ));
    }

    #[test]
    fn test_block_comment() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
/* block comment */
onflag() {
    /* another
       multi-line
       block comment */
    say "hello";
}
"#,
        ));
    }

    // ─── Event handler syntax ───────────────────────────────────────────────────

    #[test]
    fn test_onflag_event() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    say "flag!";
}
"#,
        ));
    }

    #[test]
    fn test_onclick_event() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onclick() {
    say "clicked!";
}
"#,
        ));
    }

    #[test]
    fn test_onclone_event() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onclone() {
    say "cloned!";
}
"#,
        ));
    }

    #[test]
    fn test_onkey_event() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onkey("space") {
    say "space pressed";
}
"#,
        ));
    }

    #[test]
    fn test_on_broadcast_event() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
on("my_message") {
    say "received broadcast";
}
"#,
        ));
    }

    #[test]
    fn test_onbackdrop_event() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onbackdrop("intro") {
    say "backdrop changed";
}
"#,
        ));
    }

    // ─── function declarations (replaces proc/func) ─────────────────────────────

    #[test]
    fn test_function_no_args() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
function greet() {
    say "hello";
}
onflag() {
    greet;
}
"#,
        ));
    }

    #[test]
    fn test_function_with_args() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
function greet(name) {
    say "hello " & $name;
}
onflag() {
    greet "world";
}
"#,
        ));
    }

    #[test]
    fn test_function_multiple_args() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
function add(a, b) {
    result = $a + $b;
}
onflag() {
    add 3, 4;
}
"#,
        ));
    }

    #[test]
    fn test_function_with_return() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
function double(n): value {
    return $n * 2;
}
onflag() {
    say double(5);
}
"#,
        ));
    }

    #[test]
    fn test_nowarp_function() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
nowarp function slow_loop() {
    repeat 10 {
        say "tick";
    }
}
onflag() {
    slow_loop;
}
"#,
        ));
    }

    // ─── let (replaces local) ───────────────────────────────────────────────────

    #[test]
    fn test_let_untyped() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    let x = 42;
    say x;
}
"#,
        ));
    }

    #[test]
    fn test_let_with_type_annotation() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
struct Point { x, y }
onflag() {
    let p: Point = Point { x: 1, y: 2 };
    say p.x;
}
"#,
        ));
    }

    // ─── var with type annotation ────────────────────────────────────────────────

    #[test]
    fn test_var_untyped() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
var score = 0;
onflag() {
    score = 1;
}
"#,
        ));
    }

    // ─── list with type annotation ────────────────────────────────────────────────

    #[test]
    fn test_list_untyped() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
list items;
onflag() {
    add "apple" to items;
}
"#,
        ));
    }

    #[test]
    fn test_list_typed() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
struct Item { name }
list inventory: Item;
onflag() {
    add Item { name: "sword" } to inventory;
}
"#,
        ));
    }

    // ─── Logical operators ──────────────────────────────────────────────────────

    #[test]
    fn test_logical_not() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    if (!false) {
        say "not false is true";
    }
}
"#,
        ));
    }

    #[test]
    fn test_logical_and() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 5;
    if (x > 0 && x < 10) {
        say "in range";
    }
}
"#,
        ));
    }

    #[test]
    fn test_logical_or() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 5;
    if (x < 0 || x > 3) {
        say "out of 0-3 range";
    }
}
"#,
        ));
    }

    #[test]
    fn test_complex_boolean_expr() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    a = 1;
    b = 2;
    c = 3;
    if (!(a == b) && (b < c || c > 0)) {
        say "complex";
    }
}
"#,
        ));
    }

    // ─── Floor division (div / div=) ────────────────────────────────────────────

    #[test]
    fn test_floor_div_operator() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    result = 7 div 2;
    say result;
}
"#,
        ));
    }

    #[test]
    fn test_floor_div_assign() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 10;
    x div= 3;
    say x;
}
"#,
        ));
    }

    // ─── while loop ──────────────────────────────────────────────────────────────

    #[test]
    fn test_while_loop() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    i = 0;
    while (i < 10) {
        i++;
    }
    say i;
}
"#,
        ));
    }

    #[test]
    fn test_while_break_with_stop() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    while (true) {
        stop_this_script;
    }
}
"#,
        ));
    }

    // ─── if / else if / else ─────────────────────────────────────────────────────

    #[test]
    fn test_if_else_if_else() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 5;
    if (x < 0) {
        say "negative";
    } else if (x == 0) {
        say "zero";
    } else {
        say "positive";
    }
}
"#,
        ));
    }

    #[test]
    fn test_nested_else_if_chain() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    grade = 85;
    if (grade >= 90) {
        say "A";
    } else if (grade >= 80) {
        say "B";
    } else if (grade >= 70) {
        say "C";
    } else {
        say "F";
    }
}
"#,
        ));
    }

    // ─── Compound assignment operators ──────────────────────────────────────────

    #[test]
    fn test_compound_assignments() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 10;
    x += 5;
    x -= 3;
    x *= 2;
    x /= 4;
    x %= 7;
    x &= " hello";
    say x;
}
"#,
        ));
    }

    #[test]
    fn test_increment_decrement() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    i = 0;
    i++;
    i++;
    i--;
    say i;
}
"#,
        ));
    }

    // ─── repeat / forever ───────────────────────────────────────────────────────

    #[test]
    fn test_repeat_loop() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    repeat 5 {
        say "tick";
    }
}
"#,
        ));
    }

    #[test]
    fn test_forever_loop() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    forever {
        say "forever";
    }
}
"#,
        ));
    }

    // ─── Operators ──────────────────────────────────────────────────────────────

    #[test]
    fn test_arithmetic_operators() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    a = 10;
    b = 3;
    say a + b;
    say a - b;
    say a * b;
    say a / b;
    say a div b;
    say a % b;
}
"#,
        ));
    }

    #[test]
    fn test_comparison_operators() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    a = 5;
    b = 5;
    say a == b;
    say a != b;
    say a < b;
    say a <= b;
    say a > b;
    say a >= b;
}
"#,
        ));
    }

    #[test]
    fn test_string_join_operator() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    greeting = "Hello" & ", " & "World!";
    say greeting;
}
"#,
        ));
    }

    #[test]
    fn test_in_operator() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
list fruits;
onflag() {
    add "apple" to fruits;
    if ("apple" in fruits) {
        say "found";
    }
}
"#,
        ));
    }

    // ─── Math unary operators ────────────────────────────────────────────────────

    #[test]
    fn test_math_unary_ops() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 4;
    say round 3.7;
    say abs -5;
    say floor 3.9;
    say ceil 3.1;
    say sqrt x;
    say sin 90;
    say cos 0;
    say tan 45;
    say ln 1;
    say log 100;
    say length "hello";
}
"#,
        ));
    }

    // ─── Literals ────────────────────────────────────────────────────────────────

    #[test]
    fn test_number_literals() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    say 0xFF;
    say 0b1010;
    say 0o777;
    say 3.14159;
    say -42;
}
"#,
        ));
    }

    #[test]
    fn test_boolean_literals() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    say true;
    say false;
}
"#,
        ));
    }

    #[test]
    fn test_string_literals() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    say "hello world";
    say "tab:\there";
    say "newline:\nhere";
    say "quote: \"quoted\"";
}
"#,
        ));
    }

    // ─── Struct / enum ───────────────────────────────────────────────────────────

    #[test]
    fn test_struct_definition_and_usage() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
struct Point { x, y }
onflag() {
    let p: Point = Point { x: 10, y: 20 };
    say p.x & "," & p.y;
}
"#,
        ));
    }

    #[test]
    fn test_enum_definition_and_usage() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
enum Color { Red, Green, Blue }
onflag() {
    c = Color.Red;
    say c;
}
"#,
        ));
    }

    // ─── Broadcast ───────────────────────────────────────────────────────────────

    #[test]
    fn test_broadcast() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    broadcast "start";
    broadcast_and_wait "setup";
}
"#,
        ));
    }

    // ─── Control flow blocks ─────────────────────────────────────────────────────

    #[test]
    fn test_stop_blocks() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    if (false) {
        stop_all;
    }
    if (false) {
        stop_this_script;
    }
}
"#,
        ));
    }

    #[test]
    fn test_clone_blocks() {
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onclone() {
    say "I am a clone";
}
onflag() {
    clone;
}
"#,
        ));
    }

    // ─── Multi-sprite project ────────────────────────────────────────────────────

    #[test]
    fn test_multi_sprite_project() {
        assert_compiles(&[
            stage(),
            ("project/blank.svg", BLANK_SVG),
            (
                "project/sprite1.gs",
                r#"costumes "blank.svg";
var shared_score = 0;
onflag() {
    shared_score = 0;
    say "sprite1 ready";
}
"#,
            ),
            (
                "project/sprite2.gs",
                r#"costumes "blank.svg";
onflag() {
    say "sprite2 ready";
}
"#,
            ),
        ]);
    }

    // ─── Error cases ─────────────────────────────────────────────────────────────

    #[test]
    fn test_old_proc_keyword_fails() {
        // The old `proc` keyword should no longer be recognised
        assert_fails(&sprite(
            r#"costumes "blank.svg";
proc greet {
    say "hello";
}
onflag() {
    greet;
}
"#,
        ));
    }

    #[test]
    fn test_old_local_keyword_fails() {
        // The old `local` keyword should no longer be recognised
        assert_fails(&sprite(
            r#"costumes "blank.svg";
onflag() {
    local x = 5;
    say x;
}
"#,
        ));
    }

    #[test]
    fn test_old_floor_div_operator_removed() {
        // The old `//` floor-division operator is no longer a binary operator —
        // it is now a line comment delimiter.  `x = 10 // 3` silently assigns 10
        // to `x` (the `// 3` is treated as a comment).  The canonical floor
        // division is now `div`, which compiles correctly.
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 10 div 3;
    say x;
}
"#,
        ));
    }


    #[test]
    fn test_old_and_keyword_fails() {
        // `and` is no longer a keyword; the expression should be an error.
        assert_fails(&sprite(
            r#"costumes "blank.svg";
onflag() {
    if (true and false) {
        say "should not reach";
    }
}
"#,
        ));
    }

    #[test]
    fn test_old_or_keyword_fails() {
        assert_fails(&sprite(
            r#"costumes "blank.svg";
onflag() {
    if (true or false) {
        say "should not reach";
    }
}
"#,
        ));
    }

    #[test]
    fn test_old_not_keyword_fails() {
        assert_fails(&sprite(
            r#"costumes "blank.svg";
onflag() {
    if (not false) {
        say "should not reach";
    }
}
"#,
        ));
    }

    #[test]
    fn test_old_elif_keyword_fails() {
        assert_fails(&sprite(
            r#"costumes "blank.svg";
onflag() {
    x = 5;
    if (x < 0) {
        say "neg";
    } elif (x == 0) {
        say "zero";
    }
}
"#,
        ));
    }

    #[test]
    fn test_old_until_keyword_fails() {
        assert_fails(&sprite(
            r#"costumes "blank.svg";
onflag() {
    i = 0;
    until i >= 5 {
        i++;
    }
}
"#,
        ));
    }

    #[test]
    fn test_parenthesized_proc_call_no_args() {
        // `name();` should be accepted as a proc-call statement.
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
function do_thing() {
    say "hello";
}
onflag() {
    do_thing();
}
"#,
        ));
    }

    #[test]
    fn test_parenthesized_proc_call_multiple_args() {
        // `name(a, b);` should be accepted as a proc-call statement.
        assert_compiles(&sprite(
            r#"costumes "blank.svg";
function add_and_say(a, b) {
    say $a + $b;
}
onflag() {
    add_and_say(1, 2);
}
"#,
        ));
    }

    // ─── fmt (formatter) smoke test ───────────────────────────────────────────────

    #[test]
    fn test_fmt_does_not_corrupt_valid_source() {
        use crate::fmt;
        use std::fs;

        // Write a temp file, format it, and confirm it still compiles.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("main.gs");
        let src = r#"// comment
onflag() {
    let x = 42;
    say x;
}
"#;
        fs::write(&path, src).unwrap();
        fmt::format_file(path.clone()).unwrap();
        let formatted = fs::read_to_string(&path).unwrap();
        // Formatted source should still parse cleanly when embedded in a project.
        assert_compiles(&[stage(), ("project/main.gs", &formatted)]);
    }
}
