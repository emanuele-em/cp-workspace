//{"name":"Concert Tickets","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1091","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n5 3 7 8 5\n4 8 3\n","output":"3\n8\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ConcertTickets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn binary_search(h: &[usize], target: usize)-> usize{
    let mut l = 0;
    let mut r = h.len()-1;
    while l < r{
        let m = (l+r)>>1;
        if h[m] == target{
            return m;
        }

        if h[m] < target{
            l = m+1
        } else {
            r = m
        }
    }

    return l
}
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut h = input.read_vec::<usize>(n);
    let t = input.read_vec::<usize>(m);
    let mut given = vec![false; n];
    h.sort();

    for max in t{
        let idx = binary_search(&h, max as usize);
        if !given[idx] && h[idx] == max {
            out.print_line(h[idx]);
            given[idx] = true;
        } else {
            let mut next:isize = idx as isize;
            //next
            while next < n as isize && given[next as usize] {
                println!("{}", next);
                next += 1;
            }

            let mut prev: isize = idx as isize;
            //previous
            while prev >= 0 && given[prev as usize] {
                println!("{}", prev);
                prev -= 1;
            }

            if prev < 0 && next as usize >= n {
                out.print_line(-1);
            } else if prev < 0 {
                if max >= h[next as usize] {
                    out.print_line(h[next as usize]);
                    given[next as usize] = true;
                } else {
                    out.print_line(-1);
                }
            } else if next as usize >= n {
                if max >= h[prev as usize] {
                    out.print_line(h[prev as usize]);
                    given[prev as usize] = true;
                } else {
                    out.print_line(-1);
                }
            } else {
                if max >= h[prev as usize] {
                    out.print_line(h[prev as usize]);
                    given[prev as usize] = true;
                } else if max >= h[next as usize] {
                    out.print_line(h[next as usize]);
                    given[next as usize] = true;
                } else {
                    out.print_line(-1);
                }
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
