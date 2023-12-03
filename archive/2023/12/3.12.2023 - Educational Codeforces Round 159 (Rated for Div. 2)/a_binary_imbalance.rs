//{"name":"A. Binary Imbalance","group":"Codeforces - Educational Codeforces Round 159 (Rated for Div. 2)","url":"https://codeforces.com/contest/1902/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n00\n2\n11\n2\n10\n","output":"YES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ABinaryImbalance"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let a = input.read_vec::<char>(n);
    let mut zero = 0;
    let mut cons_one = 0;
    let mut changeable = 0;
    for i in 0..a.len() {
        if a[i] == '0' {
            out.print_line("YES");
            return;
            // zero += 1;
            // changeable += cons_one;
            // cons_one = 0;
        } else {
            cons_one += 1;
        }
    }
    out.print_line("NO");
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
