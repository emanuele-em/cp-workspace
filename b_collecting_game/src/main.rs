//{"name":"B. Collecting Game","group":"Codeforces - Codeforces Round 914 (Div. 2)","url":"https://codeforces.com/contest/1904/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n20 5 1 4 2\n3\n1434 7 1442\n1\n1\n5\n999999999 999999999 999999999 1000000000 1000000000\n","output":"4 3 0 3 1\n1 0 2\n0\n4 4 4 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCollectingGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {

    let n = input.read_size();
    let a = input.read_vec::<usize>(n);
    let mut sorted = a.iter().enumerate().collect::<Vec<(usize, &usize)>>();
    sorted.sort_unstable_by_key(|x| x.1);

    let mut memo = std::collections::HashMap::new();

    for i in 0..n{
        let start = sorted[i].0;
        let score = sorted[i].1;
        let mut count = 0;
        
        for el in sorted.iter().skip(i){
            if score < *el.1{
                break;
            }
            if score >= *sorted.last().unwrap().1{
                count = sorted.len()-1;
                break;
            }
            if start == el.0{
                continue;
            }
            if score >= *el.1{
                score += el.1;
                count += 1;
            }
        }
        sorted[i+1].1 += &score;
        memo.insert(sorted[i].0, count);

    }

    for (i, el) in a.iter().enumerate(){
        out.print(format!("{} ", memo.get(&i).unwrap()));
    }

    
    out.print(format!("\n"));
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
