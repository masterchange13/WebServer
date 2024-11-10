use std::iter::Map;

/**
@Created by Mao on 2024/11/10
@Author:mao
@Github:https://github.com/masterchange13?tab=projects
@Gitee:https://gitee.com/master_change13
 
@File: test.rs
@IDE: RustRover
 
@Time: 2024/11/10 19:04
@Motto:不积跬步无以至千里，不积小流无以成江海，程序人生的精彩需要坚持不懈地积累！
@target: 大厂offer，高年薪

@@ written by GuangZhi Mao

@from:
@code target:
**/

#[test]
fn test_map()
{
    println!("testing--------------");
    let data = Map::new("1", 2);
    for i in Iterator(data){
        println!("{}", i);
    }
}