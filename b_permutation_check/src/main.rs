//{"name":"B - Permutation Check","group":"AtCoder - AtCoder Beginner Contest 205","url":"https://atcoder.jp/contests/abc205/tasks/abc205_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 1 2 4 5\n","output":"Yes\n"},{"input":"6\n3 1 4 1 5 2\n","output":"No\n"},{"input":"3\n1 2 3\n","output":"Yes\n"},{"input":"1\n1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPermutationCheck"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let bench = (1..=n).collect::<Vec<_>>();
    let mut to_check = input.read_vec::<usize>(n);
    to_check.sort_unstable();
    out.print_line(if to_check == bench { "Yes" } else { "No" });
    
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
