use srcutils::Sentinel;

fn main()
{
    let mut sent = Sentinel::new();

    // TODO: Test overflow and underflow of the needle
    let source = "add: op.u8(lhs.u8, rhs.u8)\n{\n\tlhs + rhs\n\t}\n";
    let mut points = vec![];

    for ch in source.chars()
    {
        points.push(sent.encode(sent.total_lines, sent.total_chars));
        if ch == '\n'
        {
            sent.push_newline();
        }
        else
        {
            sent.push_char();
        }
    }

    for (i, pt) in points.into_iter().enumerate()
    {
        let ch = source.chars().nth(i).unwrap();
        let mut dbg = format!("{:?}", ch);
        if ch == ' '
        {
            dbg = r"\_".to_string();
        }
        // todo make a look up table that knows when to trim ' or "
        dbg = dbg.trim_matches('"').to_string();
        println!("{:4} {:?}", dbg, sent.decode(pt));
    }
}

// TODO: Make lib for to_dbg for chars and strs that makes the above unnecessary
