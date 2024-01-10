//{"name":"C - Rotation","group":"AtCoder - AtCoder Beginner Contest 258","url":"https://atcoder.jp/contests/abc258/tasks/abc258_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\nabc\n2 2\n1 1\n2 2\n","output":"b\na\n"},{"input":"10 8\ndsuccxulnl\n2 4\n2 7\n1 2\n2 7\n1 1\n1 2\n1 3\n2 5\n","output":"c\nu\nc\nu\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRotation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_vec::<char>(n);
    let mut start_index = 0;
    for _ in 0..q{
        let t = input.read_size();
        let mut x = input.read_size();
        x = x%n;
        if t == 1{
            if start_index > x{
                start_index -= x;
            } else {
                start_index = n-(x-start_index);
            }
        } else {
            let to_end = n-start_index;
            //2
            //rem = x-to_end
            if x > to_end{
                out.print_line(s[x-to_end-1]);
            } else {
                out.print_line(s[start_index+x-1]);
            }
        }
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
