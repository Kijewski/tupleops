use std::fmt::Write;

pub(crate) fn gen_range(
    dest: &mut String,
    from: usize,
    to: usize,
    gen1: fn(dest: &mut String, to: usize) -> Result<(), std::fmt::Error>,
) -> Result<(), std::fmt::Error> {
    for to in from..=to {
        if !dest.is_empty() {
            dest.push_str("\n\n");
        }
        gen1(dest, to)?;
    }
    Ok(())
}

pub(crate) fn pattern_for(
    to: usize,
    prefix: &str,
    suffix: &str,
) -> Result<String, std::fmt::Error> {
    let mut args = String::new();
    for i in 1..=to {
        if !args.is_empty() {
            args.push(' ');
        }
        write!(args, "{}{}{},", prefix, i, suffix)?;
    }
    Ok(args)
}

pub(crate) fn pattern_for2(
    to: usize,
    prefix: &str,
    infix: &str,
    suffix: &str,
) -> Result<String, std::fmt::Error> {
    let mut args = String::new();
    for i in 1..=to {
        if !args.is_empty() {
            args.push(' ');
        }
        write!(args, "{1}{0}{2}{0}{3},", i, prefix, infix, suffix)?;
    }
    Ok(args)
}
