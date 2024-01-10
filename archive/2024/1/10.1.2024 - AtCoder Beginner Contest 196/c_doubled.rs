//{"name":"C - Doubled","group":"AtCoder - AtCoder Beginner Contest 196","url":"https://atcoder.jp/contests/abc196/tasks/abc196_c","interactive":false,"timeLimit":2000,"tests":[{"input":"33\n","output":"3\n"},{"input":"1333\n","output":"13\n"},{"input":"10000000\n","output":"999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDoubled"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let mut res = 1;

    let f = |x: usize| {
        let s = format!("{}{}", x.to_string(), x.to_string());
        s.parse::<usize>().unwrap()
    };

    while f(res) <= n {
        res+=1
    }

    out.print_line(res -1);
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
