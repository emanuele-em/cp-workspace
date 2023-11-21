//{"name":"A. Secret Sport","group":"Codeforces - Codeforces Round 908 (Div. 2)","url":"https://codeforces.com/contest/1894/problem/A","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n5\nABBAA\n3\nBBB\n7\nBBAAABA\n20\nAAAAAAAABBBAABBBBBAB\n1\nA\n13\nAAAABABBABBAB\n7\nBBBAAAA\n","output":"A\nB\nA\nB\nA\nB\nA\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASecretSport"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read::<usize>();
    let mut s: Vec<_> = input.read_vec::<char>(n);
    s.reverse();
    out.print_line(s[0]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
