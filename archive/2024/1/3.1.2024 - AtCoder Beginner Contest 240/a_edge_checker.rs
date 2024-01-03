//{"name":"A - Edge Checker","group":"AtCoder - AtCoder Beginner Contest 240","url":"https://atcoder.jp/contests/abc240/tasks/abc240_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n","output":"Yes\n"},{"input":"3 5\n","output":"No\n"},{"input":"1 10\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEdgeChecker"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read::<isize>();
    let b = input.read::<isize>();
    let diff = (a-b).abs();
    if diff == 1 || (diff == 9 && (a == 10 || b == 10)){
        out.print_line("Yes");
        return
    }
    out.print_line("No");
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
