//{"name":"C. Can I Square?","group":"Codeforces - Codeforces Round 918 (Div. 4)","url":"https://codeforces.com/contest/1915/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n9\n2\n14 2\n7\n1 2 3 4 5 6 7\n6\n1 3 5 7 9 11\n4\n2 2 2 2\n","output":"YES\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCanISquare"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let squares = input.read_vec::<usize>(n);
    let total = squares.iter().map(|x| *x as f64).sum::<f64>();
    if total > 0. && total.sqrt() - (total.sqrt() as i32) as f64 == 0.0{
        out.print_line("YES");
        return;
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
