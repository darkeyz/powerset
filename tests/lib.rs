#[cfg(test)]
use powerset::*;

#[test]
fn empty_set() {
    let set1 = String::from("");
    let set2 = String::from("\n\n");
    let input1 = InputSet {
        set: set1,
        input_type: InputType::Raw,
    };
    let input2 = InputSet {
        set: set2,
        input_type: InputType::Raw,
    };
    assert_eq!("", into_powerset(input1.set).unwrap());
    assert_eq!("", into_powerset(input2.set).unwrap());
}

#[test]
fn valid_input() {
    let set = String::from("123,456,789");
    let input = InputSet {
        set,
        input_type: InputType::Raw,
    };
    assert_eq!(
        "
123
456
789
123,456
123,789
456,789
123,456,789",
        into_powerset(input.set).unwrap()
    );
}
#[test]
#[should_panic(expected = "Invalid input/sets found!")]
fn invalid_input() {
    let set = String::from("123,456,789,");
    let input = InputSet {
        set,
        input_type: InputType::Raw,
    };
    assert_eq!(
        "
123
456
789
123,456
123,789
456,789
123,456,789",
        into_powerset(input.set).unwrap()
    );
}

#[test]
fn whitespaces() {
    let set = String::from("1,    2");
    let input = InputSet {
        set,
        input_type: InputType::Raw,
    };
    assert_eq!(
        "
1
2
1,2",
        into_powerset(input.set).unwrap()
    );
}
