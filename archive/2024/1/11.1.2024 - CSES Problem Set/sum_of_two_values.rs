//{"name":"Sum of Two Values","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1640","interactive":false,"timeLimit":1000,"tests":[{"input":"4 8\n2 7 5 1\n","output":"2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SumOfTwoValues"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let target = input.read::<usize>();
    let nums = input.read_vec::<usize>(n);
    let mut sorted = vec![];
    for (i, num) in nums.iter().enumerate(){
        sorted.push((num, i));
    }
    sorted.sort_by_key(|x| x.0);
    let mut l = 0;
    let mut r = nums.len()-1;

    while l < r{
        let sum = sorted[l].0 + sorted[r].0;
        if sum == target{
            out.print_line(format!("{} {}", sorted[r].1+1, sorted[l].1+1));
            return;
        }

        if sum > target{
            r -= 1;
        } else {
            l+=1;
        }
    }
    out.print_line("IMPOSSIBLE");


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
