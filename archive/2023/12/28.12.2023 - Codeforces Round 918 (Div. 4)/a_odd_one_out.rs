//{"name":"A. Odd One Out","group":"Codeforces - Codeforces Round 918 (Div. 4)","url":"https://codeforces.com/contest/1915/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 2\n4 3 4\n5 5 6\n7 8 8\n9 0 9\n3 6 3\n2 8 2\n5 7 7\n7 7 5\n5 7 5\n","output":"1\n3\n6\n7\n0\n6\n8\n5\n5\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AOddOneOut"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read::<usize>();
    let b = input.read::<usize>();
    let c = input.read::<usize>();
    if a == b {
        out.print_line(c);
    } else if a == c {
        out.print_line(b);
    } else {
        out.print_line(a);
    }
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
