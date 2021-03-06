use std::io::BufRead;

pub fn get_lines<T: BufRead>(input: T) -> Vec<String> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        if ["EXIT", ""].contains(&line.as_str()) {
            break;
        }

        lines.push(line);
    }

    return lines;
}

pub fn get_lines_until_exit<T: BufRead>(input: T) -> Vec<String> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        if line.as_str() == "EXIT" {
            break;
        }

        lines.push(line);
    }

    return lines;
}