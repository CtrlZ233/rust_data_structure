use std::{collections::HashMap, vec};

use crate::limitedLinearTable::stack::Stack;


// 进制转换
fn hex2oct(mut hex: usize) {
    let mut S = Stack::<usize>::new();
    while hex > 0 {
        S.push(hex % 8);
        hex /= 8;
    }
    while !S.empty() {
        print!("{}", S.pop().unwrap()); 
    }
    println!();
}


// 括号匹配
fn bracket_matching(input: String) -> bool{
    let mut S = Stack::<char>::new();
    let mut map = HashMap::<char, char>::new();
    map.insert('[', ']');
    map.insert('(', ')');
    map.insert('{', '}');

    for c in input.chars() {
        if c == '(' || c == '[' || c == '{' {
            S.push(c);
        } else if c == ')' || c == ']' || c == '}' {
            match S.pop() {
                Some(t) => {
                    if map.get(&t).unwrap() != &c {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
    true
}

fn check_legal(x: i32, y: i32, map: & Vec<Vec<char>>, visited: & Vec<Vec<i32>>) -> bool {
    if x >= map.len() as i32 || y >= map[0].len() as i32
        || x < 0 || y < 0 || map[x as usize][y as usize] == '*' 
        || visited[x as usize][y as usize] != 0{
            false
        } else {
            true
        }
}

fn next_pos(point: &Point) -> Option<Point> {
    match point.2 {
        1 => Some(Point(point.0, point.1 + 1, 1)),
        2 => Some(Point(point.0 + 1, point.1, 1)),
        3 => Some(Point(point.0, point.1 - 1, 1)),
        4 => Some(Point(point.0 - 1, point.1, 1)),
        _ => None,
    }
}

// x, y, director
#[derive(Clone, PartialEq, PartialOrd)]
struct Point(i32, i32, i32);

// 迷宫的路径搜索 *为障碍物，.为可达点
fn path_search(map:& Vec<Vec<char>>, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
    let mut visited = vec![vec![0; map[0].len()]; map.len()];

    if !(check_legal(start_x, start_y, map, &visited) && check_legal(end_x, end_y, map, &visited)) {
        println!("illegal input.");
        return;
    }

    let mut path = Vec::<Point>::new();
    let start = Point(start_x, start_y, 1);
    
    path.push(start.clone());
    let end = Point(end_x, end_y, 1);
    let mut S = Stack::<Point>::new();
    S.push(start);
    while !S.empty() {
        let mut cur = S.pop().unwrap();
        visited[cur.0  as usize][cur.1 as usize] = 1;
        if cur.0 == end.0 && cur.1 == end.1 {
            break;
        }
        let next = next_pos(&cur);
        match next {
            Some(next_p) => {
                cur.2 += 1;
                S.push(cur);
                if check_legal(next_p.0, next_p.1, map, &visited) {
                    path.push(next_p.clone());
                    S.push(next_p); 
                }
            }
            None => {
                visited[cur.0  as usize][cur.1 as usize] = 0;
                path.pop();
            }
        } 
    }
    for v in path {
        print!("({}, {})->", v.0, v.1);
    }
    println!("end");
}



#[test]
fn stack_test() {
    hex2oct(100);
    println!("{}", bracket_matching("([])}".to_string()));
    let map = vec![vec!['.','.','*'],
                                vec!['.','*','.'], 
                                vec!['.','.','.']];
    path_search(&map, 0, 0, 2, 2);
}