use crate::vstr::*;

#[test]
fn pluralize_follows_regular_and_irregular_english_rules() {
    assert_eq!(pluralize("cat", 1), "cat");
    assert_eq!(pluralize("cat", -1), "cat");
    assert_eq!(pluralize("cat", 0), "cats");
    assert_eq!(pluralize("cat", 2), "cats");
    assert_eq!(pluralize("bus", 3), "buses");
    assert_eq!(pluralize("box", 2), "boxes");
    assert_eq!(pluralize("dish", 2), "dishes");
    assert_eq!(pluralize("church", 2), "churches");
    assert_eq!(pluralize("city", 5), "cities");
    assert_eq!(pluralize("day", 2), "days");
    assert_eq!(pluralize("person", 4), "people");
    assert_eq!(pluralize("child", 2), "children");
    assert_eq!(pluralize("mouse", 2), "mice");
    assert_eq!(pluralize("", 2), "");
}

#[test]
fn singularize_reverses_regular_and_irregular_forms() {
    assert_eq!(singularize("cats"), "cat");
    assert_eq!(singularize("buses"), "bus");
    assert_eq!(singularize("boxes"), "box");
    assert_eq!(singularize("dishes"), "dish");
    assert_eq!(singularize("churches"), "church");
    assert_eq!(singularize("cities"), "city");
    assert_eq!(singularize("days"), "day");
    assert_eq!(singularize("people"), "person");
    assert_eq!(singularize("children"), "child");
    assert_eq!(singularize("mice"), "mouse");
    // Already-singular and edge inputs stay unchanged.
    assert_eq!(singularize("cat"), "cat");
    assert_eq!(singularize("class"), "class");
    assert_eq!(singularize(""), "");
}

#[test]
fn pluralize_singularize_round_trip_for_common_words() {
    for word in [
        "cat", "bus", "box", "dish", "church", "city", "day", "person", "child", "mouse",
    ] {
        let plural = pluralize(word, 2);
        assert_eq!(
            singularize(&plural),
            word,
            "round-trip failed for {word} -> {plural}"
        );
    }
}

#[test]
fn ordinalize_uses_standard_english_suffix_rules() {
    assert_eq!(ordinalize(0), "0th");
    assert_eq!(ordinalize(1), "1st");
    assert_eq!(ordinalize(2), "2nd");
    assert_eq!(ordinalize(3), "3rd");
    assert_eq!(ordinalize(4), "4th");
    assert_eq!(ordinalize(11), "11th");
    assert_eq!(ordinalize(12), "12th");
    assert_eq!(ordinalize(13), "13th");
    assert_eq!(ordinalize(21), "21st");
    assert_eq!(ordinalize(22), "22nd");
    assert_eq!(ordinalize(23), "23rd");
    assert_eq!(ordinalize(111), "111th");
    assert_eq!(ordinalize(101), "101st");
    assert_eq!(ordinalize(-1), "-1st");
    assert_eq!(ordinalize(-13), "-13th");
}

#[test]
fn deordinalize_strips_suffix_only_after_digits() {
    assert_eq!(deordinalize("1st"), "1");
    assert_eq!(deordinalize("2nd"), "2");
    assert_eq!(deordinalize("3rd"), "3");
    assert_eq!(deordinalize("4th"), "4");
    assert_eq!(deordinalize("22nd"), "22");
    assert_eq!(deordinalize("111th"), "111");
    assert_eq!(deordinalize("-13th"), "-13");
    // Non-ordinal words keep their suffix.
    assert_eq!(deordinalize("month"), "month");
    assert_eq!(deordinalize("stand"), "stand");
    assert_eq!(deordinalize(""), "");
}

#[test]
fn ordinalize_deordinalize_round_trip() {
    for value in [0_i64, 1, 2, 3, 11, 21, 101, 111, -1, -13] {
        assert_eq!(deordinalize(&ordinalize(value)), value.to_string());
    }
}

#[test]
fn humanize_makes_identifier_sentence_and_drops_id() {
    assert_eq!(humanize("employee_salary"), "Employee salary");
    assert_eq!(humanize("author_id"), "Author");
    assert_eq!(humanize("EmployeeSalary"), "Employee salary");
    assert_eq!(humanize("employee-salary"), "Employee salary");
    assert_eq!(humanize("id"), "Id");
    assert_eq!(humanize(""), "");
}

#[test]
fn titleize_capitalizes_each_word_and_drops_id() {
    assert_eq!(titleize("man_from_the_boondocks"), "Man From The Boondocks");
    assert_eq!(titleize("author_id"), "Author");
    assert_eq!(titleize("EmployeeSalary"), "Employee Salary");
    assert_eq!(titleize("employee-salary"), "Employee Salary");
    assert_eq!(titleize(""), "");
}

#[test]
fn camelize_produces_pascal_class_names() {
    assert_eq!(camelize("active_record"), "ActiveRecord");
    assert_eq!(camelize("employee-salary"), "EmployeeSalary");
    assert_eq!(camelize("employee salary"), "EmployeeSalary");
    assert_eq!(camelize(""), "");
}
