//{"name":"B. Not Quite Latin Square","group":"Codeforces - Codeforces Round 918 (Div. 4)","url":"https://codeforces.com/contest/1915/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nABC\nC?B\nBCA\nBCA\nCA?\nABC\n?AB\nBCA\nABC\n","output":"A\nB\nC\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNotQuiteLatinSquare"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let first_line = input.read_vec::<char>(3);
    let second_line = input.read_vec::<char>(3);
    let third_line = input.read_vec::<char>(3);
    let target_line = if first_line.contains(&'?') {
        first_line
    } else if second_line.contains(&'?') {
        second_line
    } else {
        third_line
    };

    if !target_line.contains(&'A'){
        out.print_line("A")
    }
    if !target_line.contains(&'B'){
        out.print_line("B")
    }
    if !target_line.contains(&'C'){
        out.print_line("C")
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
