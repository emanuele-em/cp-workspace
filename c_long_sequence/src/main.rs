//{"name":"C - Long Sequence","group":"AtCoder - AtCoder Beginner Contest 220","url":"https://atcoder.jp/contests/abc220/tasks/abc220_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 5 2\n26\n","output":"8\n"},{"input":"4\n12 34 56 78\n1000\n","output":"23\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLongSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    // 3 5 2
    // 26
    //calculate the sum
    // sum = 10
    // min = 26/10 = 2
    //calculate the start point
    // last = 2*sum = 20
    //loop until exceed
    //20 + i = 23, min += 1
    // 23 + i = 28 -> return min+1
    let n = input.read::<usize>();
    let a = input.read_vec::<usize>(n);
    let x = input.read::<usize>();
    let mut sum = a.iter().sum::<usize>(); // 10
    let initial = x/sum*a.len(); // 26/10*3 -> 2*3 = 6
    sum = sum*(x/sum); //20
    let mut res = initial; // 6
    while sum <= x {
        sum += a[res-initial];
        res += 1;
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
