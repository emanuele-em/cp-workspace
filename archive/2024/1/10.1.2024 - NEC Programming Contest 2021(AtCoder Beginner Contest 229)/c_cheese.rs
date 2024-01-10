//{"name":"C - Cheese","group":"AtCoder - NEC Programming Contest 2021(AtCoder Beginner Contest 229)","url":"https://atcoder.jp/contests/abc229/tasks/abc229_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n3 1\n4 2\n2 3\n","output":"15\n"},{"input":"4 100\n6 2\n1 5\n3 9\n8 7\n","output":"100\n"},{"input":"10 3141\n314944731 649\n140276783 228\n578012421 809\n878510647 519\n925326537 943\n337666726 611\n879137070 306\n87808915 39\n756059990 244\n228622672 291\n","output":"2357689932073\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCheese"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut w = input.read_size();
    let mut ab = vec![];
    for _ in 0..n{
        let a = input.read_size();
        let b = input.read_size();
        ab.push((a, b));
    }
    
    ab.sort();
    let mut res = 0;
    let mut i = ab.len();
    while w > 0 && i > 0{
        let (delisciusness, avalaible_grams) = ab[i-1];
        if w >= avalaible_grams{
            res += delisciusness * avalaible_grams;
            w -= avalaible_grams;
            i-=1;
        } else {
            res += delisciusness * w;
            out.print_line(res);
            return
        }
    }
    out.print_line(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
