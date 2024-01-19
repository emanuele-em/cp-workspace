//{"name":"B. Forming Triangles","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/contest/1922/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7\n1 1 1 1 1 1 1\n4\n3 2 1 3\n3\n1 2 3\n1\n1\n","output":"35\n2\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFormingTriangles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn factorial(n: isize) -> isize {
    if n == 0 {
        return 1;
    }
    (1..=n).product::<isize>() as isize
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut sticks = input.read_vec::<isize>(n);
    sticks.sort_unstable();
    let mut res: isize = 0;
    for i in 0..n as isize-3{
        for j in i+1..n as isize-2{
            let c1: isize = 2_isize.pow(sticks[i as usize] as u32);
            let c2: isize = 2_isize.pow(sticks[j as usize] as u32);
            let target = c1 + c2;
            let mut l = j+1;
            let mut r = n as isize-1;
            while l<r{
                let m = (l+r)/2;
                if sticks[m as usize] < target{
                    r = m;
                }else{
                    l = m+1;
                }
            }
            res += factorial(n as isize-l+2 as isize)/(factorial(n as isize -l as isize + 2 - 3)*6);
        }
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
