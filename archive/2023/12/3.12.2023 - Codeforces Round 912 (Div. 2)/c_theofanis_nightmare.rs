//{"name":"C. Theofanis' Nightmare","group":"Codeforces - Codeforces Round 912 (Div. 2)","url":"https://codeforces.com/contest/1903/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n6\n1 -3 7 -6 2 5\n4\n2 9 -5 -3\n8\n-3 -4 2 -5 1 10 17 23\n1\n830\n","output":"32\n4\n343\n830\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheofanisNightmare"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();


fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let mut max_sum = i64::MIN;
    for i in 0..a.len() {
        let mut current_sum = 0;
        let mut subarray_max = i64::MIN;
        for j in i..a.len() {
            current_sum += a[j];
            subarray_max = subarray_max.max(current_sum);
            if subarray_max == current_sum {
                max_sum = max_sum.max(subarray_max * ((j - i + 1) as i64));
            }
        }
    }

    out.print_line(max_sum);
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
