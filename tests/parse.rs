
#[cfg(feature = "derive")]
mod tests {
    use clipv::parser::parse;
    use clipv::TryParse;

    #[derive(TryParse, Debug, PartialEq)]
    enum Number {
        One,
        Two,
        Three
    }

    #[derive(Debug, TryParse, PartialEq)]
    enum Color {
        Red,
        Blue,
        Black,
        Yellow
    }

    #[derive(Debug, PartialEq, TryParse)]
    struct Example {
        #[try_parse]
        number: Number,
        #[try_parse]
        color: Color,
    }


    #[test]
    fn it_should_parse_and_call_the_callback() {
        let result = parse::<Example, _>(["One", "Black"].iter(), |r| r == Example {
            number: Number::One,
            color: Color::Black
        });
        assert!(result.is_ok());
        assert!(result.ok().unwrap());
    }
}
