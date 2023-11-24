//{"name":"A. Sorting with Twos","group":"Codeforces - Codeforces Round 907 (Div. 2)","url":"https://codeforces.com/contest/1891/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n5\n1 2 3 4 5\n5\n6 5 3 4 4\n9\n6 5 5 7 5 6 6 8 7\n4\n4 3 2 1\n6\n2 2 4 5 3 2\n8\n1 3 17 19 27 57 179 13\n5\n3 17 57 179 92\n10\n1 2 3 4 0 6 7 8 9 10\n","output":"YES\nYES\nYES\nNO\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASortingWithTwos"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read::<usize>();
    let mut a = input.read_vec::<usize>(n);
    let mut i = 0;
    let mut start = 0;
    while 2_isize.pow(i as u32)-1 < n as isize {
        let end:isize = (2_isize.pow(i as u32))-1;
        while start+1 <= end{
            if a[start as usize] > a[start as usize +1]{
                out.print_line("NO");
                return
            }
            start += 1;
        }
        start += 1;
        i += 1;
    }
    while start+1 < n as isize{
        if a[start as usize] > a[start as usize +1]{
            out.print_line("NO");
            return
        }
        start += 1;
    }
    out.print_line("YES");
    return
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
