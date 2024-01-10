//{"name":"C - Squared Error","group":"AtCoder - AtCoder Beginner Contest 194","url":"https://atcoder.jp/contests/abc194/tasks/abc194_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 8 4\n","output":"56\n"},{"input":"5\n-5 8 9 -4 -3\n","output":"950\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSquaredError"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_vec::<i64>(n);
    let mut freq = std::collections::HashMap::<i64, usize>::new();
    let mut unique = vec![];
    for num in a{
        if !freq.contains_key(&num){
            unique.push(num);
        }
        freq.entry(num).and_modify(|x| *x+=1).or_insert(1);
    }
    let mut res = 0_usize;
    for i in 0..unique.len(){
        for j in i..unique.len(){
            res += freq.get(&unique[i]).unwrap() * freq.get(&unique[j]).unwrap() * (unique[i]-unique[j]).pow(2) as usize;
        }
    }
    out.print_line(res)
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
