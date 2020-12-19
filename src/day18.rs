use std::iter::Peekable;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    Lit(i64),
    Op(char),
    Brackets(Vec<Expr>),
}

impl Expr {
    fn simplify_part1(&mut self) -> &mut Self {
        match self {
            Expr::Brackets(exprs) => {
                let mut exprs = exprs.iter_mut().map(|e| {
                    e.simplify_part1();
                    e
                });
                let mut accu = exprs.by_ref().next().unwrap().as_val();
                while exprs.len() != 0 {
                    let op = exprs.next().unwrap();
                    let e = exprs.next().unwrap().as_val();
                    match op {
                        Expr::Op('+') => accu += e,
                        Expr::Op('*') => accu *= e,
                        _ => panic!("Unknown operator {:?}", op),
                    }
                }
                *self = Expr::Lit(accu);
            }

            Expr::Lit(_) => {}
            Expr::Op(_) => {}
        }
        self
    }

    fn simplify_part2(&mut self) -> &mut Self {
        match self {
            Expr::Brackets(exprs) => {
                exprs.iter_mut().for_each(|e| {
                    e.simplify_part2();
                });

                // First all the sums
                while exprs.contains(&Expr::Op('+')) {
                    let mut i = 0;
                    while exprs.len() > 2 && i < exprs.len() - 2 {
                        if let [Expr::Lit(a), Expr::Op('+'), Expr::Lit(b)] = &mut exprs[i..][..3] {
                            *a += *b;
                            exprs.remove(i + 2);
                            exprs.remove(i + 1);
                        };
                        i += 1;
                    }
                }

                // Multiply everything together
                let mut exprs = exprs.iter_mut();
                let mut accu = exprs.by_ref().next().unwrap().as_val();
                while exprs.len() != 0 {
                    // Consume op
                    exprs.next();
                    let e = exprs.next().unwrap().as_val();
                    accu *= e;
                }
                *self = Expr::Lit(accu);
            }

            Expr::Lit(_) => {}
            Expr::Op(_) => {}
        }
        self
    }

    fn as_val(&self) -> i64 {
        match self {
            Expr::Lit(val) => *val,
            _ => panic!("Cannot convert to value"),
        }
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Expr> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| !c.is_whitespace()).peekable();
            Expr::Brackets(parse_expr_list(&mut chars))
        })
        .collect()
}

fn parse_expr(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    match chars.peek() {
        Some('(') => {
            // Consume bracket
            chars.by_ref().next();
            let res = Expr::Brackets(parse_expr_list(chars));
            let close_bracket = chars.next();
            assert_eq!(Some(')'), close_bracket);
            res
        }
        Some(d) if d.is_ascii_digit() => Expr::Lit(parse_lit(chars)),
        Some(c) => panic!("Unknown char {}", c),
        None => panic!("Unexpected end of iterator"),
    }
}

fn parse_expr_list(chars: &mut Peekable<impl Iterator<Item = char>>) -> Vec<Expr> {
    let mut exprs = vec![];
    exprs.push(parse_expr(chars));
    while let Some(&op) = chars.peek() {
        if op == '*' || op == '+' {
            // Consume op char
            chars.next();
            exprs.push(Expr::Op(op));
            exprs.push(parse_expr(chars))
        } else {
            break;
        }
    }
    exprs
}

fn parse_lit(chars: &mut Peekable<impl Iterator<Item = char>>) -> i64 {
    let mut buffer = String::with_capacity(5);
    while chars.peek().is_some() && chars.peek().unwrap().is_ascii_digit() {
        let d = chars.next().unwrap();
        buffer.push(d);
    }
    buffer.parse().expect("Could not parse literal buffer")
}

#[aoc(day18, part1)]
fn part1(input: &[Expr]) -> i64 {
    input
        .iter()
        .map(|expr| expr.clone().simplify_part1().as_val())
        .sum()
}

#[aoc(day18, part2)]
fn part2(input: &[Expr]) -> i64 {
    input
        .iter()
        .map(|expr| expr.clone().simplify_part2().as_val())
        .sum()
}

#[test]
fn test_formula0() {
    use Expr::*;
    let values = input_generator("1 + 2 * 3 + 4 * 5 + 6)");
    assert_eq!(
        vec![Brackets(vec![
            Lit(1),
            Op('+'),
            Lit(2),
            Op('*'),
            Lit(3),
            Op('+'),
            Lit(4),
            Op('*'),
            Lit(5),
            Op('+'),
            Lit(6)
        ])],
        values
    );
    assert_eq!(71, values[0].clone().simplify_part1().as_val());
    assert_eq!(231, values[0].clone().simplify_part2().as_val());
}

#[test]
fn test_formula1() {
    use Expr::*;
    let values = input_generator("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(
        vec![Brackets(vec![
            Lit(1),
            Op('+'),
            Brackets(vec![Lit(2), Op('*'), Lit(3)]),
            Op('+'),
            Brackets(vec![
                Lit(4),
                Op('*'),
                Brackets(vec![Lit(5), Op('+'), Lit(6)])
            ])
        ])],
        values
    );
    assert_eq!(51, values[0].clone().simplify_part1().as_val());
    assert_eq!(51, values[0].clone().simplify_part2().as_val());
}

#[test]
fn test_formula2() {
    use Expr::*;
    let values = input_generator("2 * 3 + (4 * 5)");
    assert_eq!(
        vec![Brackets(vec![
            Lit(2),
            Op('*'),
            Lit(3),
            Op('+'),
            Brackets(vec![Lit(4), Op('*'), Lit(5)])
        ])],
        values
    );
    assert_eq!(26, values[0].clone().simplify_part1().as_val());
    assert_eq!(46, values[0].clone().simplify_part2().as_val());
}

#[test]
fn test_formula3() {
    use Expr::*;
    let values = input_generator("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    assert_eq!(
        vec![Brackets(vec![
            Lit(5),
            Op('+'),
            Brackets(vec![
                Lit(8),
                Op('*'),
                Lit(3),
                Op('+'),
                Lit(9),
                Op('+'),
                Lit(3),
                Op('*'),
                Lit(4),
                Op('*'),
                Lit(3)
            ])
        ])],
        values
    );
    assert_eq!(437, values[0].clone().simplify_part1().as_val());
    assert_eq!(1445, values[0].clone().simplify_part2().as_val());
}

#[test]
fn test_formula4() {
    use Expr::*;
    let values = input_generator("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    assert_eq!(
        vec![Brackets(vec![
            Lit(5),
            Op('*'),
            Lit(9),
            Op('*'),
            Brackets(vec![
                Lit(7),
                Op('*'),
                Lit(3),
                Op('*'),
                Lit(3),
                Op('+'),
                Lit(9),
                Op('*'),
                Lit(3),
                Op('+'),
                Brackets(vec![Lit(8), Op('+'), Lit(6), Op('*'), Lit(4)])
            ])
        ])],
        values
    );
    assert_eq!(12240, values[0].clone().simplify_part1().as_val());
    assert_eq!(669060, values[0].clone().simplify_part2().as_val());
}

#[test]
fn test_formula5() {
    use Expr::*;
    let values = input_generator("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(
        vec![Brackets(vec![
            Brackets(vec![
                Brackets(vec![Lit(2), Op('+'), Lit(4), Op('*'), Lit(9)]),
                Op('*'),
                Brackets(vec![
                    Lit(6),
                    Op('+'),
                    Lit(9),
                    Op('*'),
                    Lit(8),
                    Op('+'),
                    Lit(6)
                ]),
                Op('+'),
                Lit(6)
            ]),
            Op('+'),
            Lit(2),
            Op('+'),
            Lit(4),
            Op('*'),
            Lit(2)
        ])],
        values
    );
    assert_eq!(13632, values[0].clone().simplify_part1().as_val());
    assert_eq!(23340, values[0].clone().simplify_part2().as_val());
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(26 + 437 + 12240 + 13632, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day18.txt").trim());
    assert_eq!(202553439706, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(46 + 1445 + 669060 + 23340, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day18.txt").trim());
    assert_eq!(88534268715686, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;
