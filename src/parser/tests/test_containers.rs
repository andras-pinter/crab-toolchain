use super::{Parser, error_fmt};

#[test]
fn test_parsing_with_one_container() {
    let input = indoc::indoc! {"
    @ubuntu:
        from: ubuntu:latest
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert!(ast.containers.contains_key("ubuntu".as_bytes()));
    assert_eq!(ast.containers.iter().count(), 1);
}

#[test]
fn test_parsing_default_shell() {
    let input = indoc::indoc! {"
    @ubuntu:
        from: ubuntu:latest
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert_eq!(ast.shell, "/bin/bash");
}

#[test]
fn test_parsing_with_one_container_and_shell() {
    let input = indoc::indoc! {"
    @shell:
        path: /bin/zsh

    @ubuntu:
        from: ubuntu:latest
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert_eq!(ast.shell, "/bin/zsh");
    assert!(ast.containers.contains_key("ubuntu".as_bytes()));
    assert_eq!(ast.containers.iter().count(), 1);
}

#[test]
fn test_parsing_with_multiple_container() {
    let input = indoc::indoc! {"
    @ubuntu:
        from: ubuntu:latest

    @ubuntu-focal:
        from: ubuntu:focal

    @ubuntu-bionic:
        from: ubuntu:bionic
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert!(ast.containers.contains_key("ubuntu".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-focal".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-bionic".as_bytes()));
    assert_eq!(ast.containers.iter().count(), 3);
}

#[test]
fn test_parsing_with_multiple_container_with_shell() {
    let input = indoc::indoc! {"
    @shell:
        path: /bin/zsh

    @ubuntu:
        from: ubuntu:latest

    @ubuntu-focal:
        from: ubuntu:focal

    @ubuntu-bionic:
        from: ubuntu:bionic
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert_eq!(ast.shell, "/bin/zsh");
    assert!(ast.containers.contains_key("ubuntu".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-focal".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-bionic".as_bytes()));
    assert_eq!(ast.containers.iter().count(), 3);
}

#[test]
fn test_parsing_with_container_arguments() {
    let input = indoc::indoc! {"
    @ubuntu:
        from: ubuntu:latest
        port: 80:8080
        volume: /usr/lib/:/usr/share/lib
        volume-from: cache_container
        expose: 443
        volume: /home/apple:/home/peach
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert_eq!(ast.shell, "/bin/bash");
}

#[test]
fn test_parsing_with_multiple_container_with_arguments() {
    let input = indoc::indoc! {"
    @shell:
        path: /bin/zsh

    @ubuntu:
        from: ubuntu:latest
        port: 80:8080
        volume-from: cache_container

    @ubuntu-focal:
        from: ubuntu:focal
        port: 80:8080

    @ubuntu-bionic:
        from: ubuntu:bionic
        port: 80:8080
        volume: /usr/lib/:/usr/share/lib
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert_eq!(ast.shell, "/bin/zsh");
    assert!(ast.containers.contains_key("ubuntu".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-focal".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-bionic".as_bytes()));
    assert_eq!(ast.containers.iter().count(), 3);
}

#[test]
fn test_parsing_with_multiple_container_separated_with_multiple_newline() {
    let input = indoc::indoc! {"
    @ubuntu:
        from: ubuntu:latest



    @ubuntu-focal:
        from: ubuntu:focal



    @ubuntu-bionic:
        from: ubuntu:bionic


    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_ok(), format!("Error: {:?}", result.err().map(error_fmt)));
    let ast = result.unwrap();

    assert!(ast.containers.contains_key("ubuntu".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-focal".as_bytes()));
    assert!(ast.containers.contains_key("ubuntu-bionic".as_bytes()));
    assert_eq!(ast.containers.iter().count(), 3);
}

#[test]
fn test_parsing_error() {
    let input = indoc::indoc! {"
    @shell:
        path: /bin/zsh

    @ubuntu:
        from: ubuntu:latest
        port: 80:8080
        volume-from: cache_container

    @ubuntu-focal:
        from: ubuntu:focal
        port: 80:8080
        invalid: invalid

    @ubuntu-bionic:
        from: ubuntu:bionic
        port: 80:8080
        volume: /usr/lib/:/usr/share/lib
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_err());
}

#[test]
fn test_parsing_error_at_end() {
    let input = indoc::indoc! {"
    @shell:
        path: /bin/zsh

    @ubuntu:
        from: ubuntu:latest
        port: 80:8080
        volume-from: cache_container

    @ubuntu-focal:
        from: ubuntu:focal
        port: 80:8080

    @ubuntu-bionic:
        from: ubuntu:bionic
        port: 80:8080
        volume: /usr/lib/:/usr/share/lib
        invalid: invalid
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_err());
}

#[test]
fn test_parsing_misplaced_shell() {
    let input = indoc::indoc! {"
    @ubuntu:
        from: ubuntu:latest
        port: 80:8080
        volume-from: cache_container

    @ubuntu-focal:
        from: ubuntu:focal
        port: 80:8080

    @shell:
        path: /bin/zsh

    @ubuntu-bionic:
        from: ubuntu:bionic
        port: 80:8080
        volume: /usr/lib/:/usr/share/lib
        invalid: invalid
    "};
    let result = Parser::parse(input.as_bytes());
    assert!(result.is_err());
}